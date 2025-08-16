use std::{ collections::HashMap, ops::{ Deref, DerefMut } };

use convert_case::{ Case, Casing };
use native_db::{ transaction::RwTransaction, ToKey };
use tauri::{ AppHandle, Runtime };

use crate::{
    carcosa::{ application::Application, CarcosaExt },
    models::{ keys::{ NodeKey, TemplateKey }, Node, Template },
    templates::{ types::{ PackageId, Parent }, Identifier, LayoutKind, NodeDesc, TemplateNode },
};

#[derive(Clone, Debug)]
pub struct Templates<R: Runtime>(AppHandle<R>);

#[bon::bon]
impl<R: Runtime> Templates<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self(app_handle)
    }

    fn handle(&self) -> AppHandle<R> {
        self.0.clone()
    }

    fn app(&self) -> super::Application<R> {
        self.handle().application()
    }

    #[builder]
    pub fn create(
        &self,
        #[builder(into)] name: String,

        #[builder(default = PackageId::project())] package: PackageId,

        #[builder(default = LayoutKind::Form)] layout: LayoutKind,

        #[builder(into)] description: Option<String>,

        #[builder(into)] icon: Option<String>,

        #[builder(into)] inherit: Option<Identifier>
    ) -> crate::Result<TemplateInterface<R>> {
        let friendly_id: String = name
            .clone()
            .to_case(Case::Snake)
            .chars()
            .filter_map(|c| (
                if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                    Some(c)
                } else {
                    None
                }
            ))
            .collect();
        let new_template = Template {
            id: Identifier::new(),
            friendly_id,
            name,
            package,
            icon,
            description,
            layout,
            inherit,
        };
        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        txn.insert(new_template.clone())?;
        txn.commit()?;

        self.app().events().templates().created_template(new_template.clone().into())?;
        Ok(TemplateInterface::new(self.handle(), new_template))
    }

    pub fn get_by_id(
        &self,
        id: impl Into<Identifier>
    ) -> crate::Result<Option<TemplateInterface<R>>> {
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let result = txn.get().primary::<Template>(id.into())?;
        Ok(result.and_then(|t| Some(TemplateInterface::new(self.handle(), t))))
    }

    pub fn get_by_friendly_id(
        &self,
        id: impl Into<String>
    ) -> crate::Result<Option<TemplateInterface<R>>> {
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let result = txn.get().secondary::<Template>(TemplateKey::friendly_id, id.into())?;
        Ok(result.and_then(|t| Some(TemplateInterface::new(self.handle(), t))))
    }

    pub fn all_in_package(
        &self,
        package: impl Into<PackageId>
    ) -> crate::Result<Vec<TemplateInterface<R>>> {
        let package_id: PackageId = package.into();
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let results = txn
            .scan()
            .secondary::<Template>(TemplateKey::package)?
            .range(package_id.to_key()..=package_id.to_key())?
            .filter_map(|r| r.ok().and_then(|o| Some(TemplateInterface::new(self.handle(), o))))
            .collect();
        Ok(results)
    }

    pub fn all_inherit(
        &self,
        inherit: Option<impl Into<Identifier>>
    ) -> crate::Result<Vec<TemplateInterface<R>>> {
        let inherit_key: Option<Identifier> = inherit.and_then(|s| Some(s.into()));
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let results = txn
            .scan()
            .secondary::<Template>(TemplateKey::inherit)?
            .range(inherit_key.to_key()..=inherit_key.to_key())?
            .filter_map(|r| r.ok().and_then(|o| Some(TemplateInterface::new(self.handle(), o))))
            .collect();
        Ok(results)
    }

    pub fn all_with_layout(&self, layout: LayoutKind) -> crate::Result<Vec<TemplateInterface<R>>> {
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let results = txn
            .scan()
            .secondary::<Template>(TemplateKey::inherit)?
            .range(layout.to_key()..=layout.to_key())?
            .filter_map(|r| r.ok().and_then(|o| Some(TemplateInterface::new(self.handle(), o))))
            .collect();
        Ok(results)
    }

    pub fn all(&self) -> crate::Result<Vec<TemplateInterface<R>>> {
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let results = txn
            .scan()
            .secondary::<Template>(TemplateKey::friendly_id)?
            .all()?
            .filter_map(|r| r.ok().and_then(|o| Some(TemplateInterface::new(self.handle(), o))))
            .collect();
        Ok(results)
    }

    pub fn nodes(&self) -> Nodes<R> {
        Nodes::new(self.handle())
    }
}

#[derive(Debug, Clone)]
pub struct TemplateInterface<R: Runtime> {
    handle: AppHandle<R>,
    template: Template,
}

impl<R: Runtime> Deref for TemplateInterface<R> {
    type Target = Template;
    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

impl<R: Runtime> DerefMut for TemplateInterface<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.template
    }
}

impl<R: Runtime> Into<Template> for TemplateInterface<R> {
    fn into(self) -> Template {
        self.template
    }
}

impl<R: Runtime> TemplateInterface<R> {
    pub(self) fn new(handle: AppHandle<R>, template: Template) -> Self {
        Self { handle, template }
    }

    fn app(&self) -> Application<R> {
        self.handle.application()
    }

    pub fn delete(self) -> crate::Result<Template> {
        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        let removed = txn.remove(self.template)?;
        txn.commit()?;
        Ok(removed)
    }

    pub fn save(&self) -> crate::Result<()> {
        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        let _ = txn.upsert(self.template.clone())?;
        txn.commit()?;
        Ok(())
    }

    pub fn nodes(&self) -> crate::Result<HashMap<Identifier, NodeInterface<R>>> {
        self.handle.template_nodes().in_template(self.id.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Nodes<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Nodes<R> {
    pub(self) fn new(app_handle: AppHandle<R>) -> Self {
        Self(app_handle)
    }

    fn app(&self) -> Application<R> {
        self.0.application()
    }

    pub fn get(&self, id: impl Into<Identifier>) -> crate::Result<Option<NodeInterface<R>>> {
        let id: Identifier = id.into();
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        let result = txn.get().primary::<Node>(id)?;
        Ok(result.and_then(|r| Some(NodeInterface::new(self.0.clone(), r))))
    }

    pub fn in_template(
        &self,
        template: impl Into<Identifier>
    ) -> crate::Result<HashMap<Identifier, NodeInterface<R>>> {
        let template: Identifier = template.into();
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        Ok(
            txn
                .scan()
                .secondary::<Node>(NodeKey::template)?
                .start_with(template)?
                .filter_map(|node|
                    node
                        .ok()
                        .and_then(|v| Some((v.id.clone(), NodeInterface::new(self.0.clone(), v))))
                )
                .collect()
        )
    }

    pub fn in_container(
        &self,
        parent: impl Into<Identifier>,
        collection: impl Into<String>
    ) -> crate::Result<HashMap<Identifier, NodeInterface<R>>> {
        let parent: Identifier = parent.into();
        let collection: String = collection.into();
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        Ok(
            txn
                .scan()
                .secondary::<Node>(NodeKey::parent)?
                .start_with(Parent::Child { parent, collection })?
                .filter_map(|node|
                    node
                        .ok()
                        .and_then(|v| Some((v.id.clone(), NodeInterface::new(self.0.clone(), v))))
                )
                .collect()
        )
    }

    pub fn in_root(
        &self,
        template: impl Into<Identifier>
    ) -> crate::Result<HashMap<Identifier, NodeInterface<R>>> {
        let template: Identifier = template.into();
        let db = self.app().current_database()?;
        let txn = db.r_transaction()?;
        Ok(
            txn
                .scan()
                .secondary::<Node>(NodeKey::template)?
                .start_with(template)?
                .filter_map(|node|
                    node.ok().and_then(|v| {
                        if let Parent::Root = v.parent.clone() {
                            Some((v.id.clone(), NodeInterface::new(self.0.clone(), v)))
                        } else {
                            None
                        }
                    })
                )
                .collect()
        )
    }
}

#[derive(Debug)]
pub struct NodeInterface<R: Runtime> {
    handle: AppHandle<R>,
    node: Node,
}

impl<R: Runtime> Clone for NodeInterface<R> {
    fn clone(&self) -> Self {
        Self { handle: self.handle.clone(), node: self.node.clone() }
    }
}

impl<R: Runtime> Into<Node> for NodeInterface<R> {
    fn into(self) -> Node {
        self.node
    }
}

impl<R: Runtime> NodeInterface<R> {
    pub(self) fn new(handle: AppHandle<R>, node: Node) -> Self {
        Self { handle, node }
    }

    fn app(&self) -> Application<R> {
        self.handle.application()
    }

    fn nodes(&self) -> Nodes<R> {
        self.handle.template_nodes()
    }

    pub fn template(&self) -> crate::Result<TemplateInterface<R>> {
        if let Some(template) = self.handle.templates().get_by_id(self.node().template)? {
            Ok(template)
        } else {
            Err(crate::Error::OrphanedNode(self.node.clone()))
        }
    }

    pub fn node(&self) -> Node {
        self.node.clone()
    }

    pub fn parent(&self) -> crate::Result<Option<(NodeInterface<R>, String)>> {
        match self.node().parent {
            Parent::Root => Ok(None),
            Parent::Child { parent, collection } =>
                self
                    .nodes()
                    .get(parent)
                    .and_then(|r| Ok(r.and_then(|o| Some((o, collection))))),
        }
    }

    pub fn next(&self) -> crate::Result<Option<NodeInterface<R>>> {
        if let Some(next) = self.node().next {
            if let Some(existing) = self.nodes().get(next.clone())? {
                Ok(Some(existing))
            } else {
                Err(crate::Error::BrokenNodeLink { current: self.node(), linked: next })
            }
        } else {
            Ok(None)
        }
    }

    pub fn previous(&self) -> crate::Result<Option<NodeInterface<R>>> {
        if let Some(previous) = self.node().previous {
            if let Some(existing) = self.nodes().get(previous.clone())? {
                Ok(Some(existing))
            } else {
                Err(crate::Error::BrokenNodeLink { current: self.node(), linked: previous })
            }
        } else {
            Ok(None)
        }
    }

    pub fn update(&mut self, update: NodeDesc) -> crate::Result<()> {
        let mut update = update.clone();

        // Make sure position is the same
        update.set_next(self.node().next);
        update.set_previous(self.node().previous);
        update.set_parent(self.node().parent);
        update.set_id(self.node().id);

        let new_node = Node::create(self.node().template, update);
        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        let _ = txn.upsert(new_node.clone())?;
        txn.commit()?;
        self.node = new_node;
        Ok(())
    }

    pub(self) fn stitch_neighbors(&mut self, txn: &RwTransaction) -> crate::Result<()> {
        let next_node = match self.next() {
            Err(crate::Error::BrokenNodeLink { .. }) => None,
            Err(other) => {
                return Err(other);
            }
            Ok(opt) => opt,
        };

        let previous_node = match self.previous() {
            Err(crate::Error::BrokenNodeLink { .. }) => None,
            Err(other) => {
                return Err(other);
            }
            Ok(opt) => opt,
        };

        if let Some(mut next) = next_node.clone() {
            next.node.previous = previous_node.clone().and_then(|v| Some(v.node().id));
            next.node.node.set_previous(previous_node.clone().and_then(|v| Some(v.node().id)));
            let _ = txn.upsert(next.node.clone())?;
        }

        if let Some(mut prev) = previous_node.clone() {
            prev.node.next = next_node.clone().and_then(|v| Some(v.node().id));
            prev.node.node.set_next(next_node.clone().and_then(|v| Some(v.node().id)));
            let _ = txn.upsert(prev.node.clone())?;
        }

        self.node.next = None;
        self.node.node.set_next(None);
        self.node.previous = None;
        self.node.node.set_previous(None);

        Ok(())
    }

    pub(self) fn split_neighbors(
        &mut self,
        txn: &RwTransaction,
        previous: Option<Identifier>,
        next: Option<Identifier>
    ) -> crate::Result<()> {
        let previous = match previous {
            Some(id) => self.nodes().get(id)?,
            _ => None,
        };

        let next = match next {
            Some(id) => self.nodes().get(id)?,
            _ => None,
        };

        if let Some(mut next_node) = next.clone() {
            next_node.node.previous = Some(self.node().id);
            next_node.node.node.set_previous(Some(self.node().id));
            self.node.next = Some(next_node.node().id);
            self.node.node.set_next(Some(next_node.node().id));
            let _ = txn.upsert(next_node.node.clone())?;
        } else {
            self.node.next = None;
            self.node.node.set_next(None);
        }

        if let Some(mut prev_node) = previous.clone() {
            prev_node.node.next = Some(self.node().id);
            prev_node.node.node.set_next(Some(self.node().id));
            self.node.previous = Some(prev_node.node().id);
            self.node.node.set_previous(Some(prev_node.node().id));
            let _ = txn.upsert(prev_node.node.clone())?;
        } else {
            self.node.previous = None;
            self.node.node.set_previous(None);
        }
        Ok(())
    }

    pub fn delete(mut self) -> crate::Result<()> {
        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        self.stitch_neighbors(&txn)?;
        let _ = txn.remove(self.node)?;
        txn.commit()?;
        Ok(())
    }

    pub fn move_after(&mut self, after: impl Into<Identifier>) -> crate::Result<()> {
        let after: Identifier = after.into();
        let selected = if let Some(sel) = self.nodes().get(after.clone())? {
            sel
        } else {
            return Err(crate::Error::not_found("Node", after));
        };

        self.node.parent = selected.node.parent.clone();
        self.node.node.set_parent(selected.node.parent.clone());

        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        self.stitch_neighbors(&txn)?;
        self.split_neighbors(
            &txn,
            Some(selected.node().id),
            selected.next()?.and_then(|n| Some(n.node().id))
        )?;
        let _ = txn.upsert(self.node())?;

        txn.commit()?;
        Ok(())
    }

    pub fn move_before(&mut self, before: impl Into<Identifier>) -> crate::Result<()> {
        let before: Identifier = before.into();
        let selected = if let Some(sel) = self.nodes().get(before.clone())? {
            sel
        } else {
            return Err(crate::Error::not_found("Node", before));
        };

        self.node.parent = selected.node.parent.clone();
        self.node.node.set_parent(selected.node.parent.clone());

        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        self.stitch_neighbors(&txn)?;
        self.split_neighbors(
            &txn,
            selected.previous()?.and_then(|n| Some(n.node().id)),
            Some(selected.node().id)
        )?;
        let _ = txn.upsert(self.node())?;

        txn.commit()?;
        Ok(())
    }

    pub fn move_into(&mut self, parent: Parent) -> crate::Result<()> {
        let existing = match parent.clone() {
            Parent::Root => self.nodes().in_root(self.node().template.clone())?,
            Parent::Child { parent, collection } => self.nodes().in_container(parent, collection)?,
        };

        if existing.len() > 0 {
            return Err(crate::Error::NonEmptyParent { parent: parent, id: self.node().template });
        }

        self.node.parent = parent.clone();
        self.node.node.set_parent(parent.clone());

        let db = self.app().current_database()?;
        let txn = db.rw_transaction()?;
        self.stitch_neighbors(&txn)?;
        let _ = txn.upsert(self.node())?;

        txn.commit()?;
        Ok(())
    }
}

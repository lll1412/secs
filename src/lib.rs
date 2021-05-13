use std::any::Any;
use std::cell::{RefCell, RefMut};

type VO<T> = Vec<Option<T>>;

#[derive(Default)]
pub struct World {
    entities_count: usize,
    components_vec_list: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            components_vec_list: vec![],
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        self.components_vec_list
            .iter_mut()
            .for_each(|component_vec| {
                component_vec.push_none();
            });
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity_id: usize,
        component: ComponentType,
    ) {
        for component_vec in self.components_vec_list.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity_id] = Some(component);
                return;
            };
        }
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }
        new_component_vec[entity_id] = Some(component);
        self.components_vec_list
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<VO<ComponentType>>> {
        for component_vec in self.components_vec_list.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<VO<ComponentType>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<VO<T>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None);
    }
}

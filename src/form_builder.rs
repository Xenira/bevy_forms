use core::fmt;
use std::sync::Arc;

use bevy::{
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::{debug, Bundle},
};

#[derive(Debug)]
pub struct FormBuilder {
    children: Vec<FormChild>,
}

#[derive(Debug)]
pub enum FormChild {
    Builder(FormBuilder),
    Bundle(dyn Bundle),
}

impl FormBuilder {
    pub fn build(&self, builder: ChildBuilder) {
        for child in &self.children {
            debug!("Child {:?}", child);
        }
    }

    pub fn add(&self, child: FormChild) {
        self.children.append(child);
    }
}

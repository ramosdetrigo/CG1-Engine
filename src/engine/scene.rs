#![allow(dead_code)]
use super::sphere::Sphere;
use super::light_source::LightSource;
use std::rc::Rc;
use std::cell::RefCell;


// #[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    objects: Vec<Rc<RefCell<Sphere>>>,
    lights: Vec<Rc<RefCell<LightSource>>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn add_object(&mut self, sphere: Sphere) -> Rc<RefCell<Sphere>> {
        self.objects.push( Rc::new(RefCell::new(sphere)) );
        Rc::clone( self.objects.last().unwrap() )
    }

    pub fn add_light(&mut self, light: LightSource) {
        self.lights.push( Rc::new(RefCell::new(light)) );
    }
    
    pub fn objects(&self) -> &Vec<Rc<RefCell<Sphere>>> { &self.objects }
    pub fn lights(&self) -> &Vec<Rc<RefCell<LightSource>>> { &self.lights }
}
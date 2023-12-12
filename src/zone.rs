#[derive(PartialEq, Copy, Clone)]
pub struct Uchastok<'a> {
    zone: Zone<'a>,
    pub name: &'a str,
    description: &'a str
}

#[derive(PartialEq, Copy, Clone)]
pub struct Zone<'a> {
    name: &'a str,
    observables: &'a str,
    infrastructure: &'a str,
    commentary: &'a str
}

pub struct Uchastki<'a> {
    pub uchastki: Vec<Uchastok<'a>>
}

const Y: Zone = Zone{
    name: "лес",
    observables: "нужно удобрить",
    infrastructure: "трудно подъехать",
    commentary: "все плохо"
};

const X: Uchastok = Uchastok {
    zone: Y,
    name: "боброво",
    description: "небольшой поселок"
};

impl<'a>  Uchastki<'_> {
    pub fn get_uchastki(&mut self) -> Vec<Uchastok> {
        self.uchastki.push(X);
        let ret = self.uchastki.clone();
        ret
    }

    pub fn add_uchastki(&mut self, uch: Uchastok<'a> ) -> Uchastok<'a> {
        let mut uchastki = self.uchastki.clone();
        uchastki.push(uch.clone());
        uch
    }

    /*pub fn edit_uchastok(&'a mut self, uch: Uchastok) -> Uchastok {
        let mut k: usize;
        self.uchastki.push(X);
        for i in 0..self.uchastki.len() {
            if self.uchastki[i] == uch {
                self.uchastki[i] = new_uchastok;
                return self.uchastki[i];
            }
        }
        X
    }*/

    pub fn delete_uchastok<T>(&mut self, uch: Uchastok) -> Result<u8, &str>{
        for i in 0..self.uchastki.len() {
            if self.uchastki[i] == uch {
                self.uchastki.remove(i);
                return Ok(0);
            }
        }
        Err("участок не найден")
    }
}






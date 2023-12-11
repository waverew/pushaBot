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

pub fn get_uchastki<'b>() -> Vec<Uchastok<'b>> {
    let mut uchastki = vec![];
    uchastki.push(X);
    uchastki
}

pub fn add_uchastki(uch: Uchastok) -> Uchastok {
    let mut uchastki = vec![];
    uchastki.push(uch);
    uch
}
pub fn edit_uchastok<'a>(uch: Uchastok<'a>, new_uchastok: Uchastok<'a>) -> Uchastok<'a> {
    let mut uchastki = vec![];
    let mut k: usize;
    uchastki.push(X);
    for i in 0..uchastki.len() {
        if uchastki[i] == uch {
            uchastki[i] = new_uchastok.clone();
        }
    }
    new_uchastok
}

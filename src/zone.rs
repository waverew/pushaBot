pub struct Uchastok<'a> {
    zone: Zone<'a>,
    pub name: &'a str,
    description: &'a str
}

pub struct Zone<'a> {
    name: &'a str,
    observables: &'a str,
    infrastructure: &'a str,
    commentary: &'a str
}

const y: Zone = Zone{
    name: "лес",
    observables: "нужно удобрить",
    infrastructure: "трудно подъехать",
    commentary: "все плохо"
};

const x: Uchastok = Uchastok {
    zone: y,
    name: "боброво",
    description: "небольшой поселок"
};

pub fn get_uchastki<'b>() -> Vec<Uchastok<'b>> {
    let mut uchastki = vec![];
    uchastki.push(x);
    uchastki
}

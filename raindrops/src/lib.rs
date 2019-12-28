pub fn raindrops(n: u32) -> String {
    let pling=n%3==0;
    let plang=n%5==0;
    let plong=n%7==0;
    let no_sound=n.to_string();

    if pling && plang && plong {
        return String::from("PlingPlangPlong");
    }
    else if pling && plang{
        return String::from("PlingPlang");
    }
    else if pling && plong{
        return String::from("PlingPlong");
    }
    else if plang && plong{
        return String::from("PlangPlong");
    }
    else if pling{
        return String::from("Pling");
    }
    else if plang{
        return String::from("Plang");
    }
    else if plong{
        return String::from("Plong");
    }
    else{
        return no_sound;
    }
}

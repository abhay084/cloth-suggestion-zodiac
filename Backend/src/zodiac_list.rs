pub enum ZodiacList {
    Mesh,
    Vrishabh,
    Mithun,
    Karka,
    Simha,
    Kanya,
    Tula,
    Vrischika,
    Dhanu,
    Makar,
    Kumbh,
    Meen,
}

impl ZodiacList {
    pub fn to_names(&self) -> (&'static str, &'static str) {
        match self {
            ZodiacList::Mesh => ("मेष", "Aries"),
            ZodiacList::Vrishabh => ("वृषभ", "Taurus"),
            ZodiacList::Mithun => ("मिथुन", "Gemini"),
            ZodiacList::Karka => ("कर्क", "Cancer"),
            ZodiacList::Simha => ("सिंह", "Leo"),
            ZodiacList::Kanya => ("कन्या", "Virgo"),
            ZodiacList::Tula => ("तुला", "Libra"),
            ZodiacList::Vrischika => ("वृश्चिक", "Scorpio"),
            ZodiacList::Dhanu => ("धनु", "Sagittarius"),
            ZodiacList::Makar => ("मकर", "Capricorn"),
            ZodiacList::Kumbh => ("कुम्भ", "Aquarius"),
            ZodiacList::Meen => ("मीन", "Pisces"),
        }
    }
}



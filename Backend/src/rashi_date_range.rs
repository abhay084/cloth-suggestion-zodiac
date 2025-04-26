use crate::zodiac_list::ZodiacList;

pub struct RashiDateRange {
    pub start_month: u32,
    pub start_day: u32,
    pub end_month: u32,
    pub end_day: u32,
    pub rashi: ZodiacList,
}

impl RashiDateRange {
    pub fn new(start_month: u32, start_day: u32, end_month: u32, end_day: u32, rashi: ZodiacList) -> Self {
        RashiDateRange {
            start_month,
            start_day,
            end_month,
            end_day,
            rashi,
        }
    }

    pub fn matches(&self, month: u32, day: u32) -> bool {
        if self.start_month == self.end_month {
            // Zodiac within the same month
            month == self.start_month && day >= self.start_day && day <= self.end_day
        } else {
            // Zodiac spans two months
            (month == self.start_month && day >= self.start_day) ||
                (month == self.end_month && day <= self.end_day)
        }
    }
}

pub fn find_zodiac_sign(month: u32, day: u32) -> Option<ZodiacList> {
    let rashis = vec![
        RashiDateRange::new(3, 21, 4, 19, ZodiacList::Mesh),
        RashiDateRange::new(4, 20, 5, 20, ZodiacList::Vrishabh),
        RashiDateRange::new(5, 21, 6, 20, ZodiacList::Mithun),
        RashiDateRange::new(6, 21, 7, 22, ZodiacList::Karka),
        RashiDateRange::new(7, 23, 8, 22, ZodiacList::Simha),
        RashiDateRange::new(8, 23, 9, 22, ZodiacList::Kanya),
        RashiDateRange::new(9, 23, 10, 22, ZodiacList::Tula),
        RashiDateRange::new(10, 23, 11, 21, ZodiacList::Vrischika),
        RashiDateRange::new(11, 22, 12, 21, ZodiacList::Dhanu),
        RashiDateRange::new(12, 22, 1, 19, ZodiacList::Makar),
        RashiDateRange::new(1, 20, 2, 18, ZodiacList::Kumbh),
        RashiDateRange::new(2, 19, 3, 20, ZodiacList::Meen),
    ];

    for rashi in rashis {
        if rashi.matches(month, day) {
            return Some(rashi.rashi);
        }
    }

    None
}

pub(crate) fn get_lucky_colors(rashi: ZodiacList, skin_tone: &str) -> Vec<String> {
    let base_colors = match rashi {
        ZodiacList::Mesh => vec!["Red", "Orange"],
        ZodiacList::Vrishabh => vec!["White", "Pink", "Green"],
        ZodiacList::Mithun => vec!["Green", "Yellow"],
        ZodiacList::Karka => vec!["White", "Silver", "Cream"],
        ZodiacList::Simha => vec!["Gold", "Orange"],
        ZodiacList::Kanya => vec!["Green", "Yellow"],
        ZodiacList::Tula => vec!["White", "Blue", "Pink"],
        ZodiacList::Vrischika => vec!["Red", "Maroon", "Black"],
        ZodiacList::Dhanu => vec!["Purple", "Yellow"],
        ZodiacList::Makar => vec!["Black", "Brown"],
        ZodiacList::Kumbh => vec!["Blue", "Grey"],
        ZodiacList::Meen => vec!["Sea Green", "White"],
    };

    let mut final_colors = vec![];

    for color in &base_colors {
        match skin_tone {
            "fair" => {
                if *color == "White" || *color == "Pink" || *color == "Yellow" || *color == "Sea Green" {
                    final_colors.push(format!("{} (highlight for fair skin)", color));
                }
            }
            "medium" => {
                if *color == "Green" || *color == "Blue" || *color == "Purple" || *color == "Orange" {
                    final_colors.push(format!("{} (looks good on medium skin)", color));
                }
            }
            "dark" => {
                if *color == "Red" || *color == "Gold" || *color == "Maroon" || *color == "Black" {
                    final_colors.push(format!("{} (enhances dark skin)", color));
                }
            }
            _ => {
                final_colors.push(color.to_string()); // fallback if skin tone is unknown
            }
        }
    }

    if final_colors.is_empty() {
        final_colors = base_colors.iter().map(|c| c.to_string()).collect();
    }

    final_colors
}

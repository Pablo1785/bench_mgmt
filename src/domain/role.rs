#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Role {
    Designer,

    // TIP: Every enum variant can have a different set of named fields
    Developer {
        avg_project_delay_days: u8,
        is_cobol_ninja: bool,
    },
    Finance,
    HR,
    Legal,

    // TIP: Every enum variant can have a different set of unnamed fields AKA be a tuple
    Marketing(usize, String),
    ProductManager,
    ProjectManager,
    OfficeManager,
    QA,
}

impl Role {
    fn can_maintain_a_banking_app(&self) -> bool {
        match self {
            // TIP: Pattern matching is very convenient in tandem with enums
            // TIP: `*<variable name>` is used for dereferencing
            Role::Developer {
                avg_project_delay_days,
                is_cobol_ninja,
            } => avg_project_delay_days > &10 && *is_cobol_ninja,
            _ => false,
        }
    }

    fn get_brand_power_level(&self) -> usize {
        match self {
            // TIP: Add additional restrictions on `match` branches with `if` guards
            Role::Marketing(level_of_funny, social_media_of_the_day)
                if social_media_of_the_day
                    .to_lowercase()
                    .split_whitespace()
                    .collect::<String>()
                    == "tiktok" =>
            {
                *level_of_funny
            }
            Role::Marketing(level_of_funny, _) => level_of_funny / 2,
            _ => 0,
        }
    }
}

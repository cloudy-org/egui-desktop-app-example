
struct Achievement {
    clicks: i128,

    message: &'static str
}

const ACHIEVEMENTS: [Achievement; 8] = [
    Achievement {
        clicks: 1,

        message: "Your first click, yayyyy!!!"
    },
    Achievement {
        clicks: 10,

        message: "Your 10th click, uwu."
    },
    Achievement {
        clicks: 67,

        message: "NO!"
    },
    Achievement {
        clicks: 69,

        message: "haha, nice."
    },
    Achievement {
        clicks: 100,

        message: "Your 100th click, i'm proud of you."
    },
    Achievement {
        clicks: 1000,

        message: "Congratulations, you're the 1000th visitor"
    },
    Achievement {
        clicks: 10000,

        message: "You must be really bored huh?"
    },
    Achievement {
        clicks: 100000,

        message: "god damn!"
    }
];

pub fn handle(num: i128, enabled: bool) -> Option<&'static str> {
    if enabled {
        return ACHIEVEMENTS.iter().find(|p| num == p.clicks).map(|a| a.message);
    }

    None
}

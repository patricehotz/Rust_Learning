enum Coin {
    FuefRaeppler,
    ZehnRaeppler,
    FuenfzigRaeppler,
    Franken(u16),
    Zweifraenkler,
    FuenfFraenkler,
}

impl Coin {
    fn is_old(&self) -> Option<bool> {
        let Coin::Franken(year) = self else {
            return None;
        };

        Some(year <= &1800_u16)
    }
}

fn value_in_rappen(coin: Coin) -> u16 {
    match coin {
        Coin::FuefRaeppler => 5,
        Coin::ZehnRaeppler => 10,
        Coin::FuenfzigRaeppler => 50,
        Coin::Franken(year) => {
            println!("Franken of year: {year}");
            100
        }
        Coin::Zweifraenkler => 200,
        Coin::FuenfFraenkler => 500,
    }
}

fn describe_franken(coin: Coin) -> Option<String> {
    match coin.is_old() {
        Some(c) => {
            if c {
                Some(format!("{c} is pretty old"))
            } else {
                Some(format!("{c} is relatively new."))
            }
        }
        _ => None,
    }
}

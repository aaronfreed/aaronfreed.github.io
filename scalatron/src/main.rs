use scalatron::*;

fn main() {
    let scales: Vec<Scale> = serde_json::from_reader(
        /* way out in the water see it swimming */
        std::fs::File::open("input/input.json").expect("WHERE IS MY INPUT"),
    )
    .expect("input is not valid");
    for scale in scales {
        let polarity = scale.get_polarity().unwrap();
        let mut intervals = Vec::new();
        match polarity {
            ScalePolarity::Descending => {
                for pair in scale.notes.windows(2) {
                    intervals.push(pair[1].semitones_below(pair[0]))
                }
                intervals.push(
                    scale
                        .notes
                        .first()
                        .unwrap()
                        .semitones_below(*scale.notes.last().unwrap()),
                );
            }
            ScalePolarity::Ascending => {
                for pair in scale.notes.windows(2) {
                    intervals.push(pair[1].semitones_above(pair[0]))
                }
                intervals.push(
                    scale
                        .notes
                        .first()
                        .unwrap()
                        .semitones_above(*scale.notes.last().unwrap()),
                );
            }
        }
        assert_eq!(
            intervals.iter().copied().sum::<i32>(),
            12,
            "A scale’s intervals didn’t add up to 12. Something is wrong with the input."
        );
        print!("<tr>");
        match scale.names.len() {
            1 => {
                print!(
                    "<td class=\"weight4 centred\" colspan=\"2\">{}</td>",
                    scale.names[0]
                );
            }
            2 => {
                print!(
                    "<td class=\"weight4 right\">{}</td><td class=\"weight4\">{}</td>",
                    scale.names[0], scale.names[1]
                );
            }
            _ => {
                panic!("a scale had a weird number of names, we expect 1 or 2");
            }
        }
        for (i, note) in scale.notes.iter().enumerate() {
            print!(
                "<td class=\"{borderpad}{color}weight{weight}\">{note}{invisinat}</td>",
                borderpad = if i == 0 { "leftborder leftpad15 " } else { "" },
                weight = match note.accidental {
                    -3 => 0,
                    -2 => 1,
                    -1 => 3,
                    0 => 5,
                    1 => 7,
                    2 => 8,
                    _ => 5,
                },
                color = match note.accidental {
                    -3 | -2 => "azure-translucent ",
                    -1 => "azure ",
                    0 => "",
                    1 | 2 => "orange ",
                    _ => "",
                },
                invisinat = match note.accidental {
                    0 => "<span class=\"hidden\">♮</span>",
                    _ => "",
                },
            );
        }
        for (i, interval) in intervals.iter().enumerate() {
            print!(
                "<td class=\"centred {borderpad}{color}weight{weight}\">{whole}{half}</td>",
                borderpad = if i == 0 {
                    "leftborder leftpad8 rightpad4 "
                } else {
                    "leftpad4 rightpad4 "
                },
                color = match interval {
                    1 => "azure ",
                    2 => "",
                    x if *x > 0 => "orange ",
                    _ => unreachable!(),
                },
                weight = match interval {
                    1 => 1,
                    2 => 3,
                    3 => 5,
                    4 => 7,
                    x if *x > 0 => 8,
                    _ => unreachable!(),
                },
                whole = if *interval >= 2 {
                    (interval / 2).to_string()
                } else {
                    "".to_string()
                },
                half = if interval % 2 == 0 { "" } else { "½" },
            );
        }
        println!("</tr>");
    }
}

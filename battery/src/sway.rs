use battery::*;
use chrono::Local;
use std::{
    env,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let (shell_cmd, colors): (String, [String; 6]) = {
        let mut args = env::args().take(8).collect::<Vec<String>>();
        // remove the name of the executable from args
        args.remove(0);
        (
            args.remove(0),
            args.try_into().expect(
                "Expecting 6 arguments \
                   battery_green battery_white battery_red volume time date",
            ),
        )
    };

    println!("{{ \"version\": 1 }}\n[");
    loop {
        let acpi_output: String = get_shell_output(&shell_cmd).unwrap();

        let perc = get_correct_percentage(&acpi_output).unwrap();
        let last_perc = perc;
        let stat = get_charging_status(&acpi_output);
        let mut vol = get_shell_output("/home/bhavya/programs/scripts/echovol").unwrap();
        vol = vol[8..(vol.len() - 1)].to_string();

        let (battery_fg, battery_bg): (&str, &str) = if stat {
            (&colors[0], "#00000000")
        } else {
            match perc {
                41.. => (&colors[1], "#00000000"),
                40 => (
                    {
                        if perc != last_perc {
                            _ = thread::spawn(|| {
                                get_shell_output(
                                    "swaynag -t warning -y overlay -m \"BATTERY LOW\" &",
                                )
                                .unwrap()
                            });
                        }
                        &colors[2]
                    },
                    "#00000000",
                ),
                31..40 => (&colors[2], "#00000000"),
                30 => (
                    {
                        if perc != last_perc {
                            _ = thread::spawn(|| {
                                get_shell_output(
                                    "swaynag -t warning -y overlay -m \"BATTERY VERY LOW\" &",
                                )
                                .unwrap()
                            });
                        }
                        "#ffffffff"
                    },
                    &colors[2],
                ),
                ..30 => ("#ffffffff", &colors[2]),
            }
        };

        println!(
            "[\
            {{\"name\":\"battery\",\
                \"full_text\":\" <b><tt>{}%</tt></b> \",\
                \"color\":\"{}\",\
                \"markup\":\"pango\",\
                \"background\":\"{}\",\
            }},\
            {{\"name\":\"volume\",\
                \"full_text\":\" <b><tt>{}</tt></b> \",\
                \"color\":\"{}\",\
                \"markup\":\"pango\",\
            }},\
            {{\"name\":\"time\",\
                \"full_text\":\" <b>{}</b> \",\
                \"color\":\"{}\",\
                \"markup\":\"pango\",\
            }},\
            {{\"name\":\"date\",\
                \"short_text\":\" <b>{}</b> \",\
                \"full_text\":\" <b>{}</b> \",\
                \"color\":\"{}\",\
                \"markup\":\"pango\",\
            }}],",
            //PERCENTAGE
            perc,
            battery_fg,
            battery_bg,
            //VOLUME
            vol,
            colors[3],
            //TIME
            Local::now().format("%-I:%M %p").to_string(), //time
            colors[4],
            //DATE
            Local::now().format("%a").to_string(), //weekday
            Local::now().format("%a %d.%m.%Y").to_string(), //weekday + date
            colors[5],
        );

        sleep(Duration::from_millis(250));
    }
}

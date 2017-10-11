fn main() {
    let days = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth",
                "Ninth", "Tenth", "Eleventh", "Twelfth"];
    let items = ["a Partridge in a Pear Tree",
                 "Two Turtle Doves",
                 "Three French Hens",
                 "Four Calling Birds",
                 "Five Gold Rings",
                 "Six Geese a-Laying",
                 "Seven Swans a-Swimming",
                 "Eight Maids a-Milking",
                 "Nine Ladies Dancing",
                 "Ten Lords a-Leaping",
                 "Eleven Pipers Piping",
                 "Twelve Drummers Drumming"];

    let mut count_down_day = 0;
    for day in days.iter() {
        println!();
        println!("On the {} day of Christmas my true love sent to me", day);

        count_down_day = count_down_day + 1;
        let mut count_down = 12;

        for item in items.iter().rev() {
            if count_down <= count_down_day {
                if count_down == 1 && count_down_day != 1 {
                    print!("and ");
                }
                print!("{}", item);
                if count_down == 1 {
                    println!(".");
                } else {
                    println!(",");
                }
            }

            count_down = count_down - 1;
        }
    }
}

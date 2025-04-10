use scraper::{ Html, Selector };

#[derive(Debug)]
struct SearchResult {
    title: String,
    url: String,
    desc: Option<String>,
}

pub fn parse(htm: String) {
    let document = Html::parse_document(&htm);

    let mut output: Vec<SearchResult> = Vec::new();

    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let table = document.select(&table_selector).nth(2);

    for (i, trow) in table.unwrap().select(&tr_selector).enumerate() {
        let tdata = trow.select(&td_selector).last();
        let j = i%4;
        if i < 40 {
            match j {
            0 => { 
                let selector = Selector::parse("a").unwrap();
                let input = tdata.unwrap().select(&selector).next().unwrap();

                output.push(SearchResult {
                    title: input.inner_html().to_string(),
                    url: input.value().attr("href").unwrap().to_string(),
                    desc: None,
                });

            },

            1 => {
                let text = tdata.unwrap().text().collect::<String>();
                
                if let Some(last) = output.last_mut() {
                    last.desc = Some(text.trim().to_string());
                }
            },
            _ => {},

            }
        }

    }
    for item in output {
        println!("{}  [{}]\n{}\n\n", item.title, item.url, item.desc.unwrap());
    }
}

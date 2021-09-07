use crate::{
    model::card::{Card, Rarity},
    HTTP_CLIENT,
};
use anyhow::{bail, Result};
use reqwest::{StatusCode, Url};

pub struct CardOption<T> {
    pub idol_id: Option<Vec<T>>,
    pub rarity: Option<Vec<Rarity>>,
    pub extra_type: Option<Vec<QueryExtraType>>,
}

#[derive(Debug, Clone, Copy)]
pub enum QueryExtraType {
    None,
    Pst,
    Pstr,
    Pstp,
    Fes,
    Aniv,
    FirstAniv,
    SecondAniv,
    Extra,
}

impl std::fmt::Display for QueryExtraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        stringify!(self).to_lowercase().fmt(f)
    }
}

pub async fn fetch_idol_cards_with_opt<T: Into<i32> + std::string::ToString>(
    option: &CardOption<T>,
) -> Result<Vec<Card>> {
    let idol_id_param = option
        .idol_id
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|x| ("idolId", x.to_string()))
        .collect::<Vec<_>>();

    let rarity_param = option
        .rarity
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|x| ("rarity", x.to_string()))
        .collect::<Vec<_>>();

    let extra_type_param = option
        .extra_type
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|x| ("extraType", x.to_string()))
        .collect::<Vec<_>>();

    let param = [idol_id_param, rarity_param, extra_type_param].concat();

    let resp = HTTP_CLIENT
        .lock()
        .await
        .get("https://api.matsurihi.me/mltd/v1/cards")
        .query(&param)
        .send()
        .await?;

    if resp.status() != StatusCode::OK {
        bail!("Response status code was not 200 OK")
    }
    let b = resp.bytes().await?;

    Ok(serde_json::from_slice(&b)?)
}

pub enum CardType {
    FrameNoAwakend,     // 覚醒前・枠有り
    NoFrameNoAwakend,   // 覚醒前・枠無し
    FrameAwakened,      // 覚醒後・枠有り
    NoFrameAwakened,    // 覚醒後・枠無し
    BgNoFrameNoAwakend, // BG・枠無し
    BgNoFrameAwakened,  // BG・枠有り
}

impl CardType {
    fn suffix(&self) -> &str {
        match self {
            CardType::FrameAwakened => "_0_a",
            CardType::FrameNoAwakend => "_0_b",
            CardType::NoFrameAwakened => "_1_a",
            CardType::NoFrameNoAwakend => "_1_b",
            CardType::BgNoFrameNoAwakend => "_0",
            CardType::BgNoFrameAwakened => "_1",
        }
    }

    fn path_subdir(&self) -> &str {
        match self {
            CardType::FrameAwakened => "card",
            CardType::FrameNoAwakend => "card",
            CardType::NoFrameAwakened => "card",
            CardType::NoFrameNoAwakend => "card",
            CardType::BgNoFrameNoAwakend => "card_bg",
            CardType::BgNoFrameAwakened => "card_bg",
        }
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardType::FrameAwakened => write!(f, "覚醒後・枠有"),
            CardType::FrameNoAwakend => write!(f, "覚醒前・枠有"),
            CardType::NoFrameAwakened => write!(f, "覚醒後・枠無"),
            CardType::NoFrameNoAwakend => write!(f, "覚醒前・枠無"),
            CardType::BgNoFrameAwakened => write!(f, "覚醒後"),
            CardType::BgNoFrameNoAwakend => write!(f, "覚醒前"),
        }
    }
}

pub async fn download_card_image(
    card_id: &str,
    card_type: &CardType,
    buf: &mut Vec<u8>,
) -> Result<()> {
    let mut base_url = Url::parse("https://storage.matsurihi.me")?;
    base_url.set_path(format!("/mltd/{}/", card_type.path_subdir()).as_str());
    let resource_id = format!("{}{}.png", card_id, card_type.suffix());
    let url = base_url.join(&resource_id)?;
    let resp = HTTP_CLIENT.lock().await.get(url).send().await?;
    if resp.status() != StatusCode::OK {
        bail!(format!(
            r#"URL: {}
Response status code was not 200 OK"#,
            resp.url()
        ));
    }
    let b = resp.bytes().await?;
    std::io::copy(&mut b.as_ref(), buf)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idol::Idol;
    #[tokio::test]
    async fn test_fetch_idol_cards_with_opt() {
        let card_option = CardOption {
            idol_id: Some(vec![Idol::MakabeMizuki as i32]),
            rarity: Some(vec![Rarity::SR]),
            extra_type: None,
        };
        let cards = fetch_idol_cards_with_opt(&card_option).await.unwrap();
        for card in cards {
            println!(
                "idol: {}, id: {}, card_id: {}, name: {}",
                card.idol_id as i32, &card.resource_id, &card.id, &card.name
            );
            assert_eq!(card.idol_id as i32, 44);
            assert_eq!(card.rarity, Rarity::SR);
        }
    }

    #[tokio::test]
    async fn test_download_card_image() {
        let card_id = "044miz0254";
        let mut bytes = Vec::new();
        download_card_image(card_id, &CardType::BgNoFrameNoAwakend, &mut bytes)
            .await
            .unwrap();
        let mut file = std::fs::File::create("test.png").unwrap();
        std::io::copy(&mut bytes.as_slice(), &mut file).unwrap();
    }
}

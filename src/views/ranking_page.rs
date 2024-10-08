use anyhow::Result;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Widget,
};
use sqlx::SqlitePool;

use super::components::ranking::Ranking;

#[derive(Default)]
pub struct RankingPage {
    ranking: Ranking,
}

impl Widget for &RankingPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let squarter_x = area.width / 4;
        let half_width = area.width / 2;

        let layout_vertical =
            Layout::vertical(&[Constraint::Percentage(80), Constraint::Percentage(20)])
                .flex(Flex::Center)
                .split(Rect::new(squarter_x, 0, half_width, area.height));

        self.ranking.render(layout_vertical[0], buf);
    }
}

impl RankingPage {
    pub async fn load_scores(&mut self, db: &SqlitePool) -> Result<()> {
        self.ranking = Ranking { scores: Vec::new() };
        self.ranking.get_sorted_scores(db).await?;

        Ok(())
    }
}

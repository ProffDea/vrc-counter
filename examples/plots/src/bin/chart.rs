#![feature(let_chains)]

use counter_core::{
	prisma::{mask_counter, PrismaClient, SortOrder},
	Result,
};
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "target/area-chart.png";

#[tokio::main]
async fn main() -> Result<()> {
	let client = PrismaClient::_builder().build().await?;

	let data = client
		.mask_counter()
		.find_many(Vec::new())
		.order_by(mask_counter::OrderByWithRelationParam::Date(SortOrder::Asc))
		.exec()
		.await?;

	// TODO: create area series for each mask enum variant
	// dbg!(&data);
	// dbg!(&data.len());

	if let Some(start_data) = data.first()
		&& let Some(last_data) = data.last()
	{
		let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
		drawing_area.fill(&WHITE)?;
		let mut chart_context = ChartBuilder::on(&drawing_area)
			.set_label_area_size(LabelAreaPosition::Left, 60)
			.set_label_area_size(LabelAreaPosition::Bottom, 60)
			.caption("Mask Counter", ("sans-serif", 40))
			.build_cartesian_2d(start_data.date..last_data.date, 0..data.len())?;

		chart_context
			.configure_mesh()
			.disable_x_mesh()
			.disable_y_mesh()
			.draw()?;

		// chart_context.draw_series(
		// 	AreaSeries::new(
		// 		data.iter().map(|data| (data.date, data.r#type)),
		// 		0,
		// 		RED.mix(0.2),
		// 	)
		// 	.border_style(RED),
		// )?;

		drawing_area.present()?;
	}

	Ok(())
}

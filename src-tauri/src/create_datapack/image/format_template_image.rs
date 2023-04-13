const IMAGE_TEMPLATE_COMMAND: &str = "execute positioned ~(xPosition) ~(yPosition) ~(zPosition) summon minecraft:text_display run data merge entity @s {Tags:[\"(filename)\"],text:'(imgcontents)',interpolation_start:-1,background:0,interpolation_duration:0,transformation:{scale:[(xScale)f,(yScale)f,(zScale)f]},line_width:9999999}";

pub fn format_template_image(filename: &String, command_num: i8, data: &String, scale: f32) -> String {
    let mut out = String::from(IMAGE_TEMPLATE_COMMAND);
    out = out.replace("(imgcontents)", data);
    out = out.replace("(filename)", filename);

    out = out.replace("(xScale)", &format!("{}", scale * 1.2));
    out = out.replace("(yScale)", &format!("{}", scale));
    out = out.replace("(zScale)", &format!("{}", scale));

    out = out.replace("(xPosition)", &format!("{}", f32::floor((command_num) as f32 / 2.0) * (0.03 * scale) + 0.00000001));
    out = out.replace("(yPosition)", &format!("{}", ((command_num % 2) as f32) * (0.075 * scale) + 0.00000001));
    out = out.replace("(zPosition)", "-5");


    return out;
}
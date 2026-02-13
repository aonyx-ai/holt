use leptos_floating::*;

// --- align_offset cross-axis tests ---

#[test]
fn align_offset_shifts_x_for_top_side() {
    // For Top/Bottom sides, the cross-axis is X, so align_offset should shift X
    let options = FloatingOptions {
        side: Side::Top,
        align: Align::Start,
        side_offset: 0.0,
        align_offset: 10.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 110.0, "align_offset should shift x for Top side");
    assert_eq!(pos.y, 200.0, "align_offset should NOT shift y for Top side");
}

#[test]
fn align_offset_shifts_x_for_bottom_side() {
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::Start,
        side_offset: 0.0,
        align_offset: 10.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 110.0, "align_offset should shift x for Bottom side");
    assert_eq!(
        pos.y, 240.0,
        "align_offset should NOT shift y for Bottom side"
    );
}

#[test]
fn align_offset_shifts_y_for_left_side() {
    // For Left/Right sides, the cross-axis is Y, so align_offset should shift Y
    let options = FloatingOptions {
        side: Side::Left,
        align: Align::Start,
        side_offset: 0.0,
        align_offset: 10.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 100.0,
        "align_offset should NOT shift x for Left side"
    );
    assert_eq!(pos.y, 210.0, "align_offset should shift y for Left side");
}

#[test]
fn align_offset_shifts_y_for_right_side() {
    let options = FloatingOptions {
        side: Side::Right,
        align: Align::Start,
        side_offset: 0.0,
        align_offset: 10.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 220.0,
        "align_offset should NOT shift x for Right side"
    );
    assert_eq!(pos.y, 210.0, "align_offset should shift y for Right side");
}

// --- Align::Center tests ---

#[test]
fn align_center_centers_on_bottom_side() {
    // reference_width=120, floating_width=80
    // Center offset = (120 - 80) / 2 = 20
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::Center,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 120.0,
        "Center align should center floating on reference (100 + 20)"
    );
    assert_eq!(pos.y, 240.0);
}

#[test]
fn align_center_centers_on_top_side() {
    let options = FloatingOptions {
        side: Side::Top,
        align: Align::Center,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 120.0,
        "Center align should center floating on reference (100 + (120-80)/2)"
    );
    assert_eq!(pos.y, 200.0);
}

#[test]
fn align_center_centers_on_right_side() {
    // reference_height=40, floating_height=30
    // Center offset = (40 - 30) / 2 = 5
    let options = FloatingOptions {
        side: Side::Right,
        align: Align::Center,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 220.0);
    assert_eq!(
        pos.y, 205.0,
        "Center align should center floating on reference (200 + (40-30)/2)"
    );
}

#[test]
fn align_center_centers_on_left_side() {
    let options = FloatingOptions {
        side: Side::Left,
        align: Align::Center,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 100.0);
    assert_eq!(
        pos.y, 205.0,
        "Center align should center floating on reference (200 + (40-30)/2)"
    );
}

// --- Align::End tests ---

#[test]
fn align_end_aligns_on_bottom_side() {
    // reference_width=120, floating_width=80
    // End offset = 120 - 80 = 40
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::End,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 140.0,
        "End align should align floating to end of reference (100 + 120 - 80)"
    );
    assert_eq!(pos.y, 240.0);
}

#[test]
fn align_end_aligns_on_top_side() {
    let options = FloatingOptions {
        side: Side::Top,
        align: Align::End,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(
        pos.x, 140.0,
        "End align should align floating to end of reference (100 + 120 - 80)"
    );
    assert_eq!(pos.y, 200.0);
}

#[test]
fn align_end_aligns_on_right_side() {
    // reference_height=40, floating_height=30
    // End offset = 40 - 30 = 10
    let options = FloatingOptions {
        side: Side::Right,
        align: Align::End,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 220.0);
    assert_eq!(
        pos.y, 210.0,
        "End align should align floating to end of reference (200 + 40 - 30)"
    );
}

#[test]
fn align_end_aligns_on_left_side() {
    let options = FloatingOptions {
        side: Side::Left,
        align: Align::End,
        side_offset: 0.0,
        align_offset: 0.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(pos.x, 100.0);
    assert_eq!(
        pos.y, 210.0,
        "End align should align floating to end of reference (200 + 40 - 30)"
    );
}

// --- Combined: Align + align_offset ---

#[test]
fn align_center_with_align_offset_on_bottom() {
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::Center,
        side_offset: 0.0,
        align_offset: 5.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    // Center: 100 + (120-80)/2 = 120, then + align_offset 5 = 125
    assert_eq!(pos.x, 125.0);
    assert_eq!(pos.y, 240.0);
}

#[test]
fn align_end_with_align_offset_on_right() {
    let options = FloatingOptions {
        side: Side::Right,
        align: Align::End,
        side_offset: 0.0,
        align_offset: 5.0,
    };

    let pos = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    // End: 200 + 40 - 30 = 210, then + align_offset 5 = 215
    assert_eq!(pos.x, 220.0);
    assert_eq!(pos.y, 215.0);
}

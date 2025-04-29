use colorgrad::Gradient;

mod utils;

#[test]
fn preset() {
    let g = colorgrad::preset::viridis();
    cmp_hex!(g.at(0.0), "#440154");
    cmp_hex!(g.at(0.5), "#27838e");
    cmp_hex!(g.at(1.0), "#fee825");
    cmp_hex!(g.at(f32::NAN), "#000000");

    let g = colorgrad::preset::greys();
    cmp_hex!(g.at(0.0), "#ffffff");
    cmp_hex!(g.at(1.0), "#000000");

    let g = colorgrad::preset::turbo();
    cmp_hex!(g.at(0.0), "#23171b");
    cmp_hex!(g.at(1.0), "#900c00");

    let g = colorgrad::preset::cividis();
    cmp_hex!(g.at(0.0), "#002051");
    cmp_hex!(g.at(1.0), "#fdea45");

    let g = colorgrad::preset::cubehelix_default();
    cmp_hex!(g.at(0.0), "#000000");
    cmp_hex!(g.at(1.0), "#ffffff");

    let g = colorgrad::preset::warm();
    cmp_hex!(g.at(0.0), "#6e40aa");
    cmp_hex!(g.at(1.0), "#aff05b");

    let g = colorgrad::preset::cool();
    cmp_hex!(g.at(0.0), "#6e40aa");
    cmp_hex!(g.at(1.0), "#aff05b");

    macro_rules! presets {
        ($($name:ident),+ $(,)?) => {
            $({
                let g = colorgrad::preset::$name();
                assert_eq!(g.domain(), (0.0, 1.0));
            })*
        }
    }

    presets!(
        blues,
        br_bg,
        bu_gn,
        bu_pu,
        cividis,
        cool,
        cubehelix_default,
        gn_bu,
        greens,
        greys,
        inferno,
        magma,
        or_rd,
        oranges,
        pi_yg,
        plasma,
        pr_gn,
        pu_bu,
        pu_bu_gn,
        pu_or,
        pu_rd,
        purples,
        rainbow,
        rd_bu,
        rd_gy,
        rd_pu,
        rd_yl_bu,
        rd_yl_gn,
        reds,
        sinebow,
        spectral,
        turbo,
        viridis,
        warm,
        yl_gn,
        yl_gn_bu,
        yl_or_br,
        yl_or_rd,
    );
}

#[test]
fn cyclic() {
    let g = colorgrad::preset::rainbow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());

    let g = colorgrad::preset::sinebow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());
}

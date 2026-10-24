#[macro_use]
extern crate safegl;

use safegl::Surface;

mod support;

#[test]
fn query_sequence() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::SamplesPassedQuery::new(&display) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    for _ in 0..3 {
        let params = safegl::DrawParameters {
            samples_passed_query: Some((&query).into()),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let result = query.get();
    assert!(result == 3 * 1024 * 1024); // 3 * texture dimensions

    display.assert_no_error(None);
}

#[test]
fn samples_passed() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::SamplesPassedQuery::new(&display) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            samples_passed_query: Some((&query).into()),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let result = query.get();
    assert!(result == 1024 * 1024); // texture dimensions

    display.assert_no_error(None);
}

#[test]
fn any_samples_passed() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::AnySamplesPassedQuery::new(&display, false) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            samples_passed_query: Some((&query).into()),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    assert!(query.get());

    let query2 = safegl::draw_parameters::AnySamplesPassedQuery::new(&display, false).unwrap();
    assert!(!query2.get());

    display.assert_no_error(None);
}

#[test]
fn any_samples_passed_conservative() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::AnySamplesPassedQuery::new(&display, true) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            samples_passed_query: Some((&query).into()),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    assert!(query.get());

    let query2 = safegl::draw_parameters::AnySamplesPassedQuery::new(&display, true).unwrap();
    assert!(!query2.get());

    display.assert_no_error(None);
}

#[test]
fn time_elapsed() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::TimeElapsedQuery::new(&display) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            time_elapsed_query: Some(&query),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let result = query.get();
    assert!(result >= 1);

    display.assert_no_error(None);
}

#[test]
#[ignore] // not sure about the interaction between primitives_generated and no geometry shader
fn primitives_generated() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::PrimitivesGeneratedQuery::new(&display) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            primitives_generated_query: Some(&query),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let result = query.get();
    assert_eq!(result, 2);

    display.assert_no_error(None);
}

// FIXME: add test for transform feedback query

// FIXME: add more tests for conditional rendering

#[test]
#[ignore] // FIXME: problem with query not having a type yet when passed to BeginConditionalRender
fn conditional_render_nodraw() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::AnySamplesPassedQuery::new(&display, false) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            condition: Some(safegl::draw_parameters::ConditionalRendering {
                query: (&query).into(),
                wait: true,
                per_region: false,
            }),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();

    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(0, 0, 0, 0));
        }
    }

    display.assert_no_error(None);
}

#[test]
#[ignore] // FIXME: not implemented yet
fn conditional_render_simultaneous_query() {
    //! we try to draw with a query and simultaneously use conditional
    //! rendering with the same query

    let display = support::build_display();

    let query = match safegl::draw_parameters::AnySamplesPassedQuery::new(&display, false) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    let params = safegl::DrawParameters {
        samples_passed_query: Some((&query).into()),
        condition: Some(safegl::draw_parameters::ConditionalRendering {
            query: (&query).into(),
            wait: true,
            per_region: false,
        }),
        ..Default::default()
    };

    let res =
        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params);

    match res {
        Err(safegl::DrawError::WrongQueryOperation) => (),
        _ => panic!(),
    };

    display.assert_no_error(None);
}

#[cfg(feature = "gl46")]
#[test]
fn query_to_buffer() {
    let display = support::build_display();

    let query = match safegl::draw_parameters::SamplesPassedQuery::new(&display) {
        Err(_) => return,
        Ok(q) => q,
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    {
        let params = safegl::DrawParameters {
            samples_passed_query: Some((&query).into()),
            ..Default::default()
        };

        texture
            .as_surface()
            .draw(&vb, &ib, &program, &safegl::uniforms::EmptyUniforms, &params)
            .unwrap();
    }

    let mut buffer = safegl::buffer::BufferView::empty(
        &display,
        safegl::buffer::BufferType::ArrayBuffer,
        safegl::buffer::BufferMode::Default,
    )
    .unwrap();
    if query.to_buffer_u32(buffer.as_slice()).is_err() {
        return;
    }

    let mapping = buffer.map();
    assert!(*mapping == 1024 * 1024); // texture dimensions

    display.assert_no_error(None);
}

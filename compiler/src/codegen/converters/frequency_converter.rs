use super::ConvertGenerator;
use ast::FormType;
use codegen::intrinsics;
use codegen::{globals, util};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, VectorValue};
use std::f32::consts;

pub fn frequency(generator: &mut ConvertGenerator) {
    generator.generate(FormType::Beats, &frequency_from_beats);
    generator.generate(FormType::Control, &frequency_from_control);
    generator.generate(FormType::Note, &frequency_from_note);
    generator.generate(FormType::Samples, &frequency_from_samples);
    generator.generate(FormType::Seconds, &frequency_from_seconds);
}

fn frequency_from_beats(
    context: &Context,
    module: &Module,
    builder: &mut Builder,
    val: VectorValue,
) -> VectorValue {
    builder.build_float_div(
        builder
            .build_load(&globals::get_bpm(module), "bpm")
            .into_vector_value(),
        builder.build_float_mul(val, util::get_vec_spread(context, 60.), ""),
        "",
    )
}

fn frequency_from_control(
    context: &Context,
    module: &Module,
    builder: &mut Builder,
    val: VectorValue,
) -> VectorValue {
    let pow_intrinsic = intrinsics::pow_v2f32(module);

    builder
        .build_call(
            &pow_intrinsic,
            &[&util::get_vec_spread(context, 20000.), &val],
            "",
            false,
        )
        .left()
        .unwrap()
        .into_vector_value()
}

fn frequency_from_note(
    context: &Context,
    module: &Module,
    builder: &mut Builder,
    val: VectorValue,
) -> VectorValue {
    let pow_intrinsic = intrinsics::pow_v2f32(module);

    builder.build_float_mul(
        util::get_vec_spread(context, 440.),
        builder
            .build_call(
                &pow_intrinsic,
                &[
                    &util::get_vec_spread(context, 2.),
                    &builder.build_float_div(
                        builder.build_float_sub(val, util::get_vec_spread(context, 69.), ""),
                        util::get_vec_spread(context, 12.),
                        "",
                    ),
                ],
                "",
                false,
            )
            .left()
            .unwrap()
            .into_vector_value(),
        "",
    )
}

fn frequency_from_samples(
    context: &Context,
    module: &Module,
    builder: &mut Builder,
    val: VectorValue,
) -> VectorValue {
    builder.build_float_div(
        builder
            .build_load(&globals::get_sample_rate(module), "samplerate")
            .into_vector_value(),
        val,
        "",
    )
}

fn frequency_from_seconds(
    context: &Context,
    module: &Module,
    builder: &mut Builder,
    val: VectorValue,
) -> VectorValue {
    builder.build_float_div(util::get_vec_spread(context, 1.), val, "")
}

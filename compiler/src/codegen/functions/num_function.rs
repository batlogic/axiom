use super::{Function, FunctionContext, VarArgs};
use codegen::values::{ArrayValue, NumValue, ARRAY_CAPACITY};
use codegen::{intrinsics, math, util};
use inkwell::types::VectorType;
use inkwell::values::PointerValue;
use inkwell::IntPredicate;
use mir::block;
use std::f64::consts;

pub struct ToRadFunction {}
impl Function for ToRadFunction {
    fn function_type() -> block::Function {
        block::Function::ToRad
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let num_arg = NumValue::new(args[0]);
        let result_num = NumValue::new(result);
        let original_vec = num_arg.get_vec(func.ctx.b);
        let original_form = num_arg.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &original_form);

        let mult_const = util::get_vec_spread(func.ctx.context, consts::PI / 180.);
        let new_vec = func.ctx.b.build_float_mul(original_vec, mult_const, "mult");
        result_num.set_vec(func.ctx.b, &new_vec);
    }
}

pub struct ToDegFunction {}
impl Function for ToDegFunction {
    fn function_type() -> block::Function {
        block::Function::ToDeg
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let num_arg = NumValue::new(args[0]);
        let result_num = NumValue::new(result);
        let original_vec = num_arg.get_vec(func.ctx.b);
        let original_form = num_arg.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &original_form);

        let mult_const = util::get_vec_spread(func.ctx.context, 180. / consts::PI);
        let new_vec = func.ctx.b.build_float_mul(original_vec, mult_const, "mult");
        result_num.set_vec(func.ctx.b, &new_vec);
    }
}

pub struct ClampFunction {}
impl Function for ClampFunction {
    fn function_type() -> block::Function {
        block::Function::Clamp
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let min_intrinsic = math::min_v2f64(func.ctx.module);
        let max_intrinsic = math::max_v2f64(func.ctx.module);

        let x_num = NumValue::new(args[0]);
        let min_vec = NumValue::new(args[1]).get_vec(func.ctx.b);
        let max_vec = NumValue::new(args[2]).get_vec(func.ctx.b);

        let result_num = NumValue::new(result);
        let result_form = x_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &result_form);

        let result_vec = x_num.get_vec(func.ctx.b);
        let result_vec = func
            .ctx
            .b
            .build_call(&min_intrinsic, &[&result_vec, &max_vec], "mined", true)
            .left()
            .unwrap()
            .into_vector_value();
        let result_vec = func
            .ctx
            .b
            .build_call(&max_intrinsic, &[&result_vec, &min_vec], "clamped", true)
            .left()
            .unwrap()
            .into_vector_value();
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}

pub struct PanFunction {}
impl Function for PanFunction {
    fn function_type() -> block::Function {
        block::Function::Pan
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let min_intrinsic = math::min_v2f64(func.ctx.module);
        let max_intrinsic = math::max_v2f64(func.ctx.module);
        let sqrt_intrinsic = math::sqrt_v2f64(func.ctx.module);
        let sin_intrinsic = math::sin_v2f64(func.ctx.module);

        let x_num = NumValue::new(args[0]);
        let pan_num = NumValue::new(args[1]);

        let result_num = NumValue::new(result);
        let result_form = x_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &result_form);

        let x_vec = x_num.get_vec(func.ctx.b);
        let clamped_pan = pan_num.get_vec(func.ctx.b);
        let clamped_pan = func
            .ctx
            .b
            .build_call(
                &min_intrinsic,
                &[&clamped_pan, &util::get_vec_spread(func.ctx.context, 1.)],
                "clamped",
                false,
            ).left()
            .unwrap()
            .into_vector_value();
        let clamped_pan = func
            .ctx
            .b
            .build_call(
                &max_intrinsic,
                &[&clamped_pan, &util::get_vec_spread(func.ctx.context, -1.)],
                "clamped",
                false,
            ).left()
            .unwrap()
            .into_vector_value();

        let left_index = func.ctx.context.i32_type().const_int(0, false);
        let left_pan = func
            .ctx
            .b
            .build_extract_element(&clamped_pan, &left_index, "pan.left")
            .into_float_value();
        let right_index = func.ctx.context.i32_type().const_int(1, false);
        let right_pan = func
            .ctx
            .b
            .build_extract_element(&clamped_pan, &right_index, "pan.right")
            .into_float_value();

        let base_param = func.ctx.b.build_float_add(
            func.ctx.b.build_float_mul(
                util::get_vec_spread(func.ctx.context, consts::FRAC_PI_4),
                func.ctx.b.build_float_add(
                    clamped_pan,
                    util::get_vec_spread(func.ctx.context, 1.),
                    "",
                ),
                "",
            ),
            util::get_const_vec(func.ctx.context, consts::FRAC_PI_2, 0.),
            "",
        );
        let base_sin = func
            .ctx
            .b
            .build_call(&sin_intrinsic, &[&base_param], "", true)
            .left()
            .unwrap()
            .into_vector_value();
        let base_mul = func
            .ctx
            .b
            .build_insert_element(
                &func
                    .ctx
                    .b
                    .build_insert_element(
                        &func.ctx.context.f64_type().vec_type(2).get_undef(),
                        &func.ctx.b.build_float_sub(
                            func.ctx.context.f64_type().const_float(1.),
                            left_pan,
                            "",
                        ),
                        &left_index,
                        "",
                    ).into_vector_value(),
                &func.ctx.b.build_float_add(
                    func.ctx.context.f64_type().const_float(1.),
                    right_pan,
                    "",
                ),
                &right_index,
                "",
            ).into_vector_value();
        let base_vec = func.ctx.b.build_float_mul(base_mul, base_sin, "");

        let multiplier_vec = func
            .ctx
            .b
            .build_call(
                &sqrt_intrinsic,
                &[&func.ctx.b.build_float_div(
                    base_vec,
                    util::get_vec_spread(func.ctx.context, 2.),
                    "",
                )],
                "",
                false,
            ).left()
            .unwrap()
            .into_vector_value();
        let result_vec = func.ctx.b.build_float_mul(x_vec, multiplier_vec, "");
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}

pub struct CombineFunction {}
impl Function for CombineFunction {
    fn function_type() -> block::Function {
        block::Function::Combine
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let left_num = NumValue::new(args[0]);
        let right_num = NumValue::new(args[1]);
        let result_num = NumValue::new(result);
        let result_form = left_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &result_form);

        let left_vec = left_num.get_vec(func.ctx.b);
        let right_vec = right_num.get_vec(func.ctx.b);
        let shuffle_vec = VectorType::const_vector(&[
            &func.ctx.context.i32_type().const_int(0, false),
            &func.ctx.context.i32_type().const_int(3, false),
        ]);
        let shuffled_vec = func
            .ctx
            .b
            .build_shuffle_vector(&left_vec, &right_vec, &shuffle_vec, "");
        result_num.set_vec(func.ctx.b, &shuffled_vec);
    }
}

pub struct MixFunction {}
impl Function for MixFunction {
    fn function_type() -> block::Function {
        block::Function::Mix
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let a_num = NumValue::new(args[0]);
        let b_num = NumValue::new(args[1]);
        let mix_num = NumValue::new(args[2]);
        let result_num = NumValue::new(result);
        let result_form = a_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &result_form);

        let a_vec = a_num.get_vec(func.ctx.b);
        let b_vec = b_num.get_vec(func.ctx.b);
        let mix_vec = mix_num.get_vec(func.ctx.b);

        let result_vec = func.ctx.b.build_float_add(
            func.ctx
                .b
                .build_float_mul(func.ctx.b.build_float_sub(b_vec, a_vec, ""), mix_vec, ""),
            a_vec,
            "",
        );
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}

pub struct SequenceFunction {}
impl Function for SequenceFunction {
    fn function_type() -> block::Function {
        block::Function::Sequence
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let eucrem_intrinsic = intrinsics::eucrem_v2i32(func.ctx.module);

        let varargs = varargs.unwrap();

        let index_num = NumValue::new(args[0]);
        let result_num = NumValue::new(result);

        let first_num =
            NumValue::new(varargs.at(func.ctx.context.i32_type().const_int(0, false), func.ctx.b));
        let first_form = first_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &first_form);

        // ensure the index is within the range (i.e number of varargs)
        let vararg_count = varargs.len(func.ctx.b);
        let vararg_count =
            func.ctx
                .b
                .build_int_z_extend(vararg_count, func.ctx.context.i32_type(), "");
        let vararg_count_vec = func
            .ctx
            .b
            .build_insert_element(
                &func.ctx.context.i32_type().vec_type(2).get_undef(),
                &vararg_count,
                &func.ctx.context.i32_type().const_int(0, false),
                "",
            ).into_vector_value();
        let vararg_count_vec = func
            .ctx
            .b
            .build_insert_element(
                &vararg_count_vec,
                &vararg_count,
                &func.ctx.context.i32_type().const_int(1, false),
                "",
            ).into_vector_value();

        let index_vec = index_num.get_vec(func.ctx.b);
        let index_int_vec = func.ctx.b.build_float_to_signed_int(
            index_vec,
            func.ctx.context.i32_type().vec_type(2),
            "",
        );
        let safe_index = func
            .ctx
            .b
            .build_call(
                &eucrem_intrinsic,
                &[&index_int_vec, &vararg_count_vec],
                "",
                true,
            ).left()
            .unwrap()
            .into_vector_value();

        let left_index = func
            .ctx
            .b
            .build_extract_element(
                &safe_index,
                &func.ctx.context.i32_type().const_int(0, false),
                "",
            ).into_int_value();
        let right_index = func
            .ctx
            .b
            .build_extract_element(
                &safe_index,
                &func.ctx.context.i32_type().const_int(1, false),
                "",
            ).into_int_value();

        let left_vec = NumValue::new(varargs.at(left_index, func.ctx.b)).get_vec(func.ctx.b);
        let right_vec = NumValue::new(varargs.at(right_index, func.ctx.b)).get_vec(func.ctx.b);

        let shuffle_vec = VectorType::const_vector(&[
            &func.ctx.context.i32_type().const_int(0, false),
            &func.ctx.context.i32_type().const_int(3, false),
        ]);
        let result_vec = func
            .ctx
            .b
            .build_shuffle_vector(&left_vec, &right_vec, &shuffle_vec, "");
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}

pub struct MixdownFunction {}
impl Function for MixdownFunction {
    fn function_type() -> block::Function {
        block::Function::Mixdown
    }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {
        let in_array = ArrayValue::new(args[0]);
        let result_num = NumValue::new(result);

        let in_bitmap = in_array.get_bitmap(func.ctx.b);

        // put the first num's form into the result
        let first_num = NumValue::new(
            in_array.get_item_ptr(func.ctx.b, func.ctx.context.i32_type().const_int(0, false)),
        );
        let first_num_form = first_num.get_form(func.ctx.b);
        result_num.set_form(func.ctx.b, &first_num_form);

        let loop_check_block = func
            .ctx
            .context
            .append_basic_block(&func.ctx.func, "loopcheck");
        let loop_run_block = func
            .ctx
            .context
            .append_basic_block(&func.ctx.func, "looprun");
        let item_active_true_block = func
            .ctx
            .context
            .append_basic_block(&func.ctx.func, "itemactive.true");
        let loop_continue_block = func
            .ctx
            .context
            .append_basic_block(&func.ctx.func, "loopcontinue");

        let result_vec = func
            .ctx
            .allocb
            .build_alloca(&func.ctx.context.f64_type().vec_type(2), "resultvec.ptr");
        func.ctx
            .b
            .build_store(&result_vec, &util::get_vec_spread(func.ctx.context, 0.));

        let index_ptr = func
            .ctx
            .allocb
            .build_alloca(&func.ctx.context.i32_type(), "index.ptr");
        func.ctx
            .b
            .build_store(&index_ptr, &func.ctx.context.i32_type().const_int(0, false));
        func.ctx.b.build_unconditional_branch(&loop_check_block);

        func.ctx.b.position_at_end(&loop_check_block);
        let current_index = func.ctx.b.build_load(&index_ptr, "index").into_int_value();
        let index_cond = func.ctx.b.build_int_compare(
            IntPredicate::ULT,
            current_index,
            func.ctx
                .context
                .i32_type()
                .const_int(ARRAY_CAPACITY as u64, false),
            "indexcond",
        );
        func.ctx
            .b
            .build_conditional_branch(&index_cond, &loop_run_block, &loop_continue_block);

        func.ctx.b.position_at_end(&loop_run_block);
        let next_index = func.ctx.b.build_int_nuw_add(
            current_index,
            func.ctx.context.i32_type().const_int(1, false),
            "nextindex",
        );
        func.ctx.b.build_store(&index_ptr, &next_index);
        let item_active = util::get_bit(func.ctx.b, in_bitmap, current_index);
        func.ctx.b.build_conditional_branch(
            &item_active,
            &item_active_true_block,
            &loop_check_block,
        );

        func.ctx.b.position_at_end(&item_active_true_block);
        let item_num = NumValue::new(in_array.get_item_ptr(func.ctx.b, current_index));
        let item_vec = item_num.get_vec(func.ctx.b);
        func.ctx.b.build_store(
            &result_vec,
            &func.ctx.b.build_float_add(
                item_vec,
                func.ctx
                    .b
                    .build_load(&result_vec, "resultvec")
                    .into_vector_value(),
                "addedvec",
            ),
        );
        func.ctx.b.build_unconditional_branch(&loop_check_block);

        func.ctx.b.position_at_end(&loop_continue_block);
        let result_vec = func
            .ctx
            .b
            .build_load(&result_vec, "resultvec")
            .into_vector_value();
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}

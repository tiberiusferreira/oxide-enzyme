; ModuleID = 'main.7rcbfp3g-cgu.0'
source_filename = "main.7rcbfp3g-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"std::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::Opaque" = type {}
%"std::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i64* }]*, i64 }, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void (i64**)*, i64, i64, i32 (i64**)*, i32 (i64**)*, i32 (i64**)* } { void (i64**)* @_ZN4core3ptr13drop_in_place17h08086275706499c8E, i64 8, i64 8, i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h86cae5132c3d1707E", i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h86cae5132c3d1707E", i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9c2a928dfb99cf6bE" }, align 8, !dbg !0
@alloc1 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"Hello, world! " }>, align 1
@alloc3 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [14 x i8] }>, <{ [14 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [8 x i8] c"\0E\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h2c116db5891d3546E(void ()* nonnull %f) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !38 {
start:
  %0 = alloca { i8*, i32 }, align 8
  %f.dbg.spill = alloca void ()*, align 8
  %result.dbg.spill = alloca {}, align 1
  %_5 = alloca {}, align 1
  %_3 = alloca {}, align 1
  call void @llvm.dbg.declare(metadata {}* %result.dbg.spill, metadata !46, metadata !DIExpression()), !dbg !52
  store void ()* %f, void ()** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata void ()** %f.dbg.spill, metadata !45, metadata !DIExpression()), !dbg !53
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h03752571149a459dE(void ()* nonnull %f), !dbg !54
  br label %bb1, !dbg !54

bb1:                                              ; preds = %start
; invoke core::hint::black_box
  invoke void @_ZN4core4hint9black_box17h1b76f0af3b5a2a39E()
          to label %bb2 unwind label %cleanup, !dbg !55

bb2:                                              ; preds = %bb1
  ret void, !dbg !56

bb3:                                              ; preds = %cleanup
  br label %bb4, !dbg !57

bb4:                                              ; preds = %bb3
  %1 = bitcast { i8*, i32 }* %0 to i8**, !dbg !58
  %2 = load i8*, i8** %1, align 8, !dbg !58
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1, !dbg !58
  %4 = load i32, i32* %3, align 8, !dbg !58
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0, !dbg !58
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1, !dbg !58
  resume { i8*, i32 } %6, !dbg !58

cleanup:                                          ; preds = %bb1
  %7 = landingpad { i8*, i32 }
          cleanup
  %8 = extractvalue { i8*, i32 } %7, 0
  %9 = extractvalue { i8*, i32 } %7, 1
  %10 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %8, i8** %10, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %9, i32* %11, align 8
  br label %bb3
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17h41dc5fab2cc34e6eE(void ()* nonnull %main, i64 %argc, i8** %argv) unnamed_addr #1 !dbg !59 {
start:
  %argv.dbg.spill = alloca i8**, align 8
  %argc.dbg.spill = alloca i64, align 8
  %main.dbg.spill = alloca void ()*, align 8
  %_7 = alloca i64*, align 8
  store void ()* %main, void ()** %main.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata void ()** %main.dbg.spill, metadata !67, metadata !DIExpression()), !dbg !71
  store i64 %argc, i64* %argc.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.dbg.spill, metadata !68, metadata !DIExpression()), !dbg !72
  store i8** %argv, i8*** %argv.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.dbg.spill, metadata !69, metadata !DIExpression()), !dbg !73
  %0 = bitcast i64** %_7 to void ()**, !dbg !74
  store void ()* %main, void ()** %0, align 8, !dbg !74
  %_4.0 = bitcast i64** %_7 to {}*, !dbg !75
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17hebea83fb59fe572eE({}* nonnull align 1 %_4.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void (i64**)*, i64, i64, i32 (i64**)*, i32 (i64**)*, i32 (i64**)* }* @vtable.0 to [3 x i64]*), i64 %argc, i8** %argv), !dbg !76
  br label %bb1, !dbg !76

bb1:                                              ; preds = %start
  ret i64 %1, !dbg !77
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h86cae5132c3d1707E"(i64** noalias readonly align 8 dereferenceable(8) %_1) unnamed_addr #2 !dbg !78 {
start:
  %_1.dbg.spill = alloca i64**, align 8
  store i64** %_1, i64*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64*** %_1.dbg.spill, metadata !84, metadata !DIExpression(DW_OP_deref)), !dbg !85
  %0 = bitcast i64** %_1 to void ()**, !dbg !86
  %_3 = load void ()*, void ()** %0, align 8, !dbg !86, !nonnull !4
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h2c116db5891d3546E(void ()* nonnull %_3), !dbg !87
  br label %bb1, !dbg !87

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %1 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97a7ffb8f66dfce9E"(), !dbg !87
  br label %bb2, !dbg !87

bb2:                                              ; preds = %bb1
  ret i32 %1, !dbg !88
}

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h6a2ea0f5235c9cdaE(i8* noalias readonly align 1 dereferenceable(1) %self) unnamed_addr #2 !dbg !89 {
start:
  %self.dbg.spill = alloca i8*, align 8
  store i8* %self, i8** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !102, metadata !DIExpression()), !dbg !103
  %_2 = load i8, i8* %self, align 1, !dbg !104
  %0 = zext i8 %_2 to i32, !dbg !104
  ret i32 %0, !dbg !105
}

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define internal { i8*, i64* } @_ZN4core3fmt10ArgumentV13new17hd3a0c717a5eb2f86E(float* noalias readonly align 4 dereferenceable(4) %x, i1 (float*, %"std::fmt::Formatter"*)* nonnull %f) unnamed_addr #1 !dbg !106 {
start:
  %0 = alloca %"core::fmt::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)*, align 8
  %f.dbg.spill = alloca i1 (float*, %"std::fmt::Formatter"*)*, align 8
  %x.dbg.spill = alloca float*, align 8
  %2 = alloca { i8*, i64* }, align 8
  store float* %x, float** %x.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata float** %x.dbg.spill, metadata !161, metadata !DIExpression()), !dbg !165
  store i1 (float*, %"std::fmt::Formatter"*)* %f, i1 (float*, %"std::fmt::Formatter"*)** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i1 (float*, %"std::fmt::Formatter"*)** %f.dbg.spill, metadata !162, metadata !DIExpression()), !dbg !166
  %3 = bitcast i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %1 to i1 (float*, %"std::fmt::Formatter"*)**, !dbg !167
  store i1 (float*, %"std::fmt::Formatter"*)* %f, i1 (float*, %"std::fmt::Formatter"*)** %3, align 8, !dbg !167
  %_3 = load i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)*, i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %1, align 8, !dbg !167, !nonnull !4
  br label %bb1, !dbg !167

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::Opaque"** %0 to float**, !dbg !168
  store float* %x, float** %4, align 8, !dbg !168
  %_5 = load %"core::fmt::Opaque"*, %"core::fmt::Opaque"** %0, align 8, !dbg !168, !nonnull !4
  br label %bb2, !dbg !168

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i64* }* %2 to %"core::fmt::Opaque"**, !dbg !169
  store %"core::fmt::Opaque"* %_5, %"core::fmt::Opaque"** %5, align 8, !dbg !169
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 1, !dbg !169
  %7 = bitcast i64** %6 to i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)**, !dbg !169
  store i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)* %_3, i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %7, align 8, !dbg !169
  %8 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 0, !dbg !170
  %9 = load i8*, i8** %8, align 8, !dbg !170, !nonnull !4
  %10 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 1, !dbg !170
  %11 = load i64*, i64** %10, align 8, !dbg !170, !nonnull !4
  %12 = insertvalue { i8*, i64* } undef, i8* %9, 0, !dbg !170
  %13 = insertvalue { i8*, i64* } %12, i64* %11, 1, !dbg !170
  ret { i8*, i64* } %13, !dbg !170
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h20fccb459b893337E(%"std::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i64* }]* noalias nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #2 !dbg !171 {
start:
  %args.dbg.spill = alloca { [0 x { i8*, i64* }]*, i64 }, align 8
  %pieces.dbg.spill = alloca { [0 x { [0 x i8]*, i64 }]*, i64 }, align 8
  %_4 = alloca { i64*, i64 }, align 8
  %1 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %1, align 8
  %2 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, i32 0, i32 1
  store i64 %pieces.1, i64* %2, align 8
  call void @llvm.dbg.declare(metadata { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, metadata !237, metadata !DIExpression()), !dbg !239
  %3 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, i32 0, i32 0
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, i32 0, i32 1
  store i64 %args.1, i64* %4, align 8
  call void @llvm.dbg.declare(metadata { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, metadata !238, metadata !DIExpression()), !dbg !240
  %5 = bitcast { i64*, i64 }* %_4 to {}**, !dbg !241
  store {}* null, {}** %5, align 8, !dbg !241
  %6 = bitcast %"std::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*, !dbg !242
  %7 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %6, i32 0, i32 0, !dbg !242
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %7, align 8, !dbg !242
  %8 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %6, i32 0, i32 1, !dbg !242
  store i64 %pieces.1, i64* %8, align 8, !dbg !242
  %9 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %0, i32 0, i32 3, !dbg !242
  %10 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0, !dbg !242
  %11 = load i64*, i64** %10, align 8, !dbg !242
  %12 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1, !dbg !242
  %13 = load i64, i64* %12, align 8, !dbg !242
  %14 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %9, i32 0, i32 0, !dbg !242
  store i64* %11, i64** %14, align 8, !dbg !242
  %15 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %9, i32 0, i32 1, !dbg !242
  store i64 %13, i64* %15, align 8, !dbg !242
  %16 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %0, i32 0, i32 5, !dbg !242
  %17 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %16, i32 0, i32 0, !dbg !242
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %17, align 8, !dbg !242
  %18 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %16, i32 0, i32 1, !dbg !242
  store i64 %args.1, i64* %18, align 8, !dbg !242
  ret void, !dbg !243
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9c2a928dfb99cf6bE"(i64** %_1) unnamed_addr #2 !dbg !244 {
start:
  %_1.dbg.spill = alloca i64**, align 8
  %_2 = alloca {}, align 1
  store i64** %_1, i64*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64*** %_1.dbg.spill, metadata !253, metadata !DIExpression()), !dbg !258
  call void @llvm.dbg.declare(metadata {}* %_2, metadata !254, metadata !DIExpression()), !dbg !258
  %0 = load i64*, i64** %_1, align 8, !dbg !258, !nonnull !4
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17he03076b2f3e69f12E(i64* nonnull %0), !dbg !258
  br label %bb1, !dbg !258

bb1:                                              ; preds = %start
  ret i32 %1, !dbg !258
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h03752571149a459dE(void ()* nonnull %_1) unnamed_addr #2 !dbg !259 {
start:
  %_1.dbg.spill = alloca void ()*, align 8
  %_2 = alloca {}, align 1
  store void ()* %_1, void ()** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata void ()** %_1.dbg.spill, metadata !261, metadata !DIExpression()), !dbg !265
  call void @llvm.dbg.declare(metadata {}* %_2, metadata !262, metadata !DIExpression()), !dbg !265
  call void %_1(), !dbg !265
  br label %bb1, !dbg !265

bb1:                                              ; preds = %start
  ret void, !dbg !265
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17he03076b2f3e69f12E(i64* nonnull %0) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !266 {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
  call void @llvm.dbg.declare(metadata i64** %_1, metadata !270, metadata !DIExpression()), !dbg !272
  call void @llvm.dbg.declare(metadata {}* %_2, metadata !271, metadata !DIExpression()), !dbg !272
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h86cae5132c3d1707E"(i64** noalias readonly align 8 dereferenceable(8) %_1)
          to label %bb1 unwind label %cleanup, !dbg !272

bb1:                                              ; preds = %start
  br label %bb2, !dbg !272

bb2:                                              ; preds = %bb1
  ret i32 %2, !dbg !272

bb3:                                              ; preds = %cleanup
  br label %bb4, !dbg !272

bb4:                                              ; preds = %bb3
  %3 = bitcast { i8*, i32 }* %1 to i8**, !dbg !272
  %4 = load i8*, i8** %3, align 8, !dbg !272
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1, !dbg !272
  %6 = load i32, i32* %5, align 8, !dbg !272
  %7 = insertvalue { i8*, i32 } undef, i8* %4, 0, !dbg !272
  %8 = insertvalue { i8*, i32 } %7, i32 %6, 1, !dbg !272
  resume { i8*, i32 } %8, !dbg !272

cleanup:                                          ; preds = %start
  %9 = landingpad { i8*, i32 }
          cleanup
  %10 = extractvalue { i8*, i32 } %9, 0
  %11 = extractvalue { i8*, i32 } %9, 1
  %12 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %10, i8** %12, align 8
  %13 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %11, i32* %13, align 8
  br label %bb3
}

; core::ptr::drop_in_place
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr13drop_in_place17h08086275706499c8E(i64** %_1) unnamed_addr #2 !dbg !273 {
start:
  %_1.dbg.spill = alloca i64**, align 8
  %0 = alloca {}, align 1
  store i64** %_1, i64*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64*** %_1.dbg.spill, metadata !279, metadata !DIExpression()), !dbg !282
  ret void, !dbg !282
}

; core::hint::black_box
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core4hint9black_box17h1b76f0af3b5a2a39E() unnamed_addr #2 !dbg !283 {
start:
  %dummy = alloca {}, align 1
  call void @llvm.dbg.declare(metadata {}* %dummy, metadata !289, metadata !DIExpression()), !dbg !290
  call void asm sideeffect "", "r,~{memory},~{dirflag},~{fpsr},~{flags}"({}* %dummy), !dbg !291, !srcloc !292
  ret void, !dbg !293
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97a7ffb8f66dfce9E"() unnamed_addr #2 !dbg !294 {
start:
  %self.dbg.spill = alloca {}, align 1
  call void @llvm.dbg.declare(metadata {}* %self.dbg.spill, metadata !301, metadata !DIExpression()), !dbg !302
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h217f181c02ce20e5E"(i8 0), !dbg !303
  br label %bb1, !dbg !303

bb1:                                              ; preds = %start
  ret i32 %0, !dbg !304
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h217f181c02ce20e5E"(i8 %0) unnamed_addr #2 !dbg !305 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
  call void @llvm.dbg.declare(metadata i8* %self, metadata !312, metadata !DIExpression()), !dbg !313
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h6a2ea0f5235c9cdaE(i8* noalias readonly align 1 dereferenceable(1) %self), !dbg !314
  br label %bb1, !dbg !314

bb1:                                              ; preds = %start
  ret i32 %1, !dbg !315
}

; main::main
; Function Attrs: uwtable
define internal void @_ZN4main4main17h2d6c3d678af9e020E() unnamed_addr #1 !dbg !316 {
start:
  %arg0.dbg.spill = alloca float*, align 8
  %_17 = alloca i32*, align 8
  %_16 = alloca [1 x { i8*, i64* }], align 8
  %_9 = alloca %"std::fmt::Arguments", align 8
  %_7 = alloca float, align 4
  %_4 = alloca float, align 4
  %result = alloca float, align 4
  call void @llvm.dbg.declare(metadata float* %result, metadata !320, metadata !DIExpression()), !dbg !326
  store float 2.000000e+00, float* %_4, align 4, !dbg !327
  store float 3.000000e+00, float* %_7, align 4, !dbg !328
; call main::simple_mul
  %0 = call float @_ZN4main10simple_mul17he69584a08c5f72d6E(float* align 4 dereferenceable(4) %_4, float* align 4 dereferenceable(4) %_7), !dbg !329
  store float %0, float* %result, align 4, !dbg !329
  br label %bb1, !dbg !329

bb1:                                              ; preds = %start
  %1 = bitcast i32** %_17 to float**, !dbg !330
  store float* %result, float** %1, align 8, !dbg !330
  %2 = bitcast i32** %_17 to float**, !dbg !330
  %arg0 = load float*, float** %2, align 8, !dbg !330, !nonnull !4
  store float* %arg0, float** %arg0.dbg.spill, align 8, !dbg !330
  call void @llvm.dbg.declare(metadata float** %arg0.dbg.spill, metadata !322, metadata !DIExpression()), !dbg !331
; call core::fmt::ArgumentV1::new
  %3 = call { i8*, i64* } @_ZN4core3fmt10ArgumentV13new17hd3a0c717a5eb2f86E(float* noalias readonly align 4 dereferenceable(4) %arg0, i1 (float*, %"std::fmt::Formatter"*)* nonnull @"_ZN4core3fmt5float52_$LT$impl$u20$core..fmt..Display$u20$for$u20$f32$GT$3fmt17h48ec592c75336fefE"), !dbg !331
  %_20.0 = extractvalue { i8*, i64* } %3, 0, !dbg !331
  %_20.1 = extractvalue { i8*, i64* } %3, 1, !dbg !331
  br label %bb2, !dbg !331

bb2:                                              ; preds = %bb1
  %4 = bitcast [1 x { i8*, i64* }]* %_16 to { i8*, i64* }*, !dbg !331
  %5 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %4, i32 0, i32 0, !dbg !331
  store i8* %_20.0, i8** %5, align 8, !dbg !331
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %4, i32 0, i32 1, !dbg !331
  store i64* %_20.1, i64** %6, align 8, !dbg !331
  %_13.0 = bitcast [1 x { i8*, i64* }]* %_16 to [0 x { i8*, i64* }]*, !dbg !330
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h20fccb459b893337E(%"std::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_9, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @alloc2 to [0 x { [0 x i8]*, i64 }]*), i64 2, [0 x { i8*, i64* }]* noalias nonnull readonly align 8 %_13.0, i64 1), !dbg !330
  br label %bb3, !dbg !330

bb3:                                              ; preds = %bb2
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h99bbe21b81bb4d97E(%"std::fmt::Arguments"* noalias nocapture dereferenceable(48) %_9), !dbg !330
  br label %bb4, !dbg !330

bb4:                                              ; preds = %bb3
  ret void, !dbg !332
}

; main::simple_mul
; Function Attrs: uwtable
define internal float @_ZN4main10simple_mul17he69584a08c5f72d6E(float* align 4 dereferenceable(4) %left, float* align 4 dereferenceable(4) %right) unnamed_addr #1 !dbg !333 {
start:
  %right.dbg.spill = alloca float*, align 8
  %left.dbg.spill = alloca float*, align 8
  store float* %left, float** %left.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata float** %left.dbg.spill, metadata !338, metadata !DIExpression()), !dbg !340
  store float* %right, float** %right.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata float** %right.dbg.spill, metadata !339, metadata !DIExpression()), !dbg !341
  %_3 = load float, float* %left, align 4, !dbg !342
  %_4 = load float, float* %right, align 4, !dbg !343
  %0 = fmul float %_3, %_4, !dbg !342
  ret float %0, !dbg !344
}

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; Function Attrs: nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #4

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17hebea83fb59fe572eE({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64, i8**) unnamed_addr #1

; core::fmt::float::<impl core::fmt::Display for f32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt5float52_$LT$impl$u20$core..fmt..Display$u20$for$u20$f32$GT$3fmt17h48ec592c75336fefE"(float* noalias readonly align 4 dereferenceable(4), %"std::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h99bbe21b81bb4d97E(%"std::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h41dc5fab2cc34e6eE(void ()* @_ZN4main4main17h2d6c3d678af9e020E, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #3 = { nounwind uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #4 = { nounwind readnone speculatable willreturn }
attributes #5 = { "frame-pointer"="all" "target-cpu"="core2" }

!llvm.module.flags = !{!14, !15, !16, !17}
!llvm.dbg.cu = !{!18}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "vtable", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !5, identifier: "vtable")
!4 = !{}
!5 = !DICompositeType(tag: DW_TAG_structure_type, name: "closure-0", scope: !6, file: !2, size: 64, align: 64, elements: !9, templateParams: !4, identifier: "3deec66e689072d8d219ca5f0767a781")
!6 = !DINamespace(name: "lang_start", scope: !7)
!7 = !DINamespace(name: "rt", scope: !8)
!8 = !DINamespace(name: "std", scope: null)
!9 = !{!10}
!10 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !5, file: !2, baseType: !11, size: 64, align: 64)
!11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !12, size: 64, align: 64, dwarfAddressSpace: 0)
!12 = !DISubroutineType(types: !13)
!13 = !{null}
!14 = !{i32 7, !"PIC Level", i32 2}
!15 = !{i32 7, !"PIE Level", i32 2}
!16 = !{i32 2, !"Dwarf Version", i32 2}
!17 = !{i32 2, !"Debug Info Version", i32 3}
!18 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !19, producer: "clang LLVM (rustc version 1.50.0-nightly (1c389ffef 2020-11-24))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !20, globals: !37)
!19 = !DIFile(filename: "../example/src/main.rs/@/main.7rcbfp3g-cgu.0", directory: "/Users/tiberio/Documents/github/oxide-c-api/enzyme")
!20 = !{!21, !28}
!21 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Result", scope: !22, file: !2, baseType: !24, size: 8, align: 8, flags: DIFlagEnumClass, elements: !25)
!22 = !DINamespace(name: "result", scope: !23)
!23 = !DINamespace(name: "core", scope: null)
!24 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!25 = !{!26, !27}
!26 = !DIEnumerator(name: "Ok", value: 0)
!27 = !DIEnumerator(name: "Err", value: 1)
!28 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !29, file: !2, baseType: !24, size: 8, align: 8, flags: DIFlagEnumClass, elements: !32)
!29 = !DINamespace(name: "v1", scope: !30)
!30 = !DINamespace(name: "rt", scope: !31)
!31 = !DINamespace(name: "fmt", scope: !23)
!32 = !{!33, !34, !35, !36}
!33 = !DIEnumerator(name: "Left", value: 0)
!34 = !DIEnumerator(name: "Right", value: 1)
!35 = !DIEnumerator(name: "Center", value: 2)
!36 = !DIEnumerator(name: "Unknown", value: 3)
!37 = !{!0}
!38 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(),()>", linkageName: "_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h2c116db5891d3546E", scope: !40, file: !39, line: 121, type: !42, scopeLine: 121, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !49, retainedNodes: !44)
!39 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "a66d3ea15c41bfcbfadc8617be007fe2")
!40 = !DINamespace(name: "backtrace", scope: !41)
!41 = !DINamespace(name: "sys_common", scope: !8)
!42 = !DISubroutineType(types: !43)
!43 = !{null, !11}
!44 = !{!45, !46}
!45 = !DILocalVariable(name: "f", arg: 1, scope: !38, file: !39, line: 121, type: !11)
!46 = !DILocalVariable(name: "result", scope: !47, file: !39, line: 125, type: !48, align: 1)
!47 = distinct !DILexicalBlock(scope: !38, file: !39, line: 125, column: 5)
!48 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!49 = !{!50, !51}
!50 = !DITemplateTypeParameter(name: "F", type: !11)
!51 = !DITemplateTypeParameter(name: "T", type: !48)
!52 = !DILocation(line: 125, column: 9, scope: !47)
!53 = !DILocation(line: 121, column: 43, scope: !38)
!54 = !DILocation(line: 125, column: 18, scope: !38)
!55 = !DILocation(line: 128, column: 5, scope: !47)
!56 = !DILocation(line: 131, column: 2, scope: !38)
!57 = !DILocation(line: 131, column: 1, scope: !38)
!58 = !DILocation(line: 121, column: 1, scope: !38)
!59 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h41dc5fab2cc34e6eE", scope: !7, file: !60, line: 60, type: !61, scopeLine: 60, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !70, retainedNodes: !66)
!60 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "a29dbe91f6c44a4e9c1b3c06440e8785")
!61 = !DISubroutineType(types: !62)
!62 = !{!63, !11, !63, !64}
!63 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!64 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !65, size: 64, align: 64, dwarfAddressSpace: 0)
!65 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 0)
!66 = !{!67, !68, !69}
!67 = !DILocalVariable(name: "main", arg: 1, scope: !59, file: !60, line: 61, type: !11)
!68 = !DILocalVariable(name: "argc", arg: 2, scope: !59, file: !60, line: 62, type: !63)
!69 = !DILocalVariable(name: "argv", arg: 3, scope: !59, file: !60, line: 63, type: !64)
!70 = !{!51}
!71 = !DILocation(line: 61, column: 5, scope: !59)
!72 = !DILocation(line: 62, column: 5, scope: !59)
!73 = !DILocation(line: 63, column: 5, scope: !59)
!74 = !DILocation(line: 66, column: 10, scope: !59)
!75 = !DILocation(line: 66, column: 9, scope: !59)
!76 = !DILocation(line: 65, column: 5, scope: !59)
!77 = !DILocation(line: 70, column: 2, scope: !59)
!78 = distinct !DISubprogram(name: "{{closure}}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h86cae5132c3d1707E", scope: !6, file: !60, line: 66, type: !79, scopeLine: 66, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !70, retainedNodes: !83)
!79 = !DISubroutineType(types: !80)
!80 = !{!81, !82}
!81 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!82 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&closure-0", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 0)
!83 = !{!84}
!84 = !DILocalVariable(name: "main", scope: !78, file: !60, line: 61, type: !11, align: 8)
!85 = !DILocation(line: 61, column: 5, scope: !78)
!86 = !DILocation(line: 66, column: 77, scope: !78)
!87 = !DILocation(line: 66, column: 18, scope: !78)
!88 = !DILocation(line: 66, column: 91, scope: !78)
!89 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h6a2ea0f5235c9cdaE", scope: !91, file: !90, line: 429, type: !98, scopeLine: 429, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !4, retainedNodes: !101)
!90 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/process/process_common.rs", directory: "", checksumkind: CSK_MD5, checksum: "0e90531be854973cc338b563f3ec214e")
!91 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !92, file: !2, size: 8, align: 8, elements: !96, templateParams: !4, identifier: "763c3540bbc200a676e26f1286dd9f23")
!92 = !DINamespace(name: "process_common", scope: !93)
!93 = !DINamespace(name: "process", scope: !94)
!94 = !DINamespace(name: "unix", scope: !95)
!95 = !DINamespace(name: "sys", scope: !8)
!96 = !{!97}
!97 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !91, file: !2, baseType: !24, size: 8, align: 8)
!98 = !DISubroutineType(types: !99)
!99 = !{!81, !100}
!100 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::unix::process::process_common::ExitCode", baseType: !91, size: 64, align: 64, dwarfAddressSpace: 0)
!101 = !{!102}
!102 = !DILocalVariable(name: "self", arg: 1, scope: !89, file: !90, line: 429, type: !100)
!103 = !DILocation(line: 429, column: 19, scope: !89)
!104 = !DILocation(line: 430, column: 9, scope: !89)
!105 = !DILocation(line: 431, column: 6, scope: !89)
!106 = distinct !DISubprogram(name: "new<f32>", linkageName: "_ZN4core3fmt10ArgumentV13new17hd3a0c717a5eb2f86E", scope: !108, file: !107, line: 267, type: !153, scopeLine: 267, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !163, retainedNodes: !160)
!107 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "8383a2f1b977481a5e40ad3226309813")
!108 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentV1", scope: !31, file: !2, size: 128, align: 64, elements: !109, templateParams: !4, identifier: "d2c90fa46196226b67729fe9298157d")
!109 = !{!110, !113}
!110 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !108, file: !2, baseType: !111, size: 64, align: 64)
!111 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::::Opaque", baseType: !112, size: 64, align: 64, dwarfAddressSpace: 0)
!112 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", file: !2, align: 8, elements: !4, identifier: "e9a59f198265abbc360b0ea240f956fa")
!113 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !108, file: !2, baseType: !114, size: 64, align: 64, offset: 64)
!114 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !115, size: 64, align: 64, dwarfAddressSpace: 0)
!115 = !DISubroutineType(types: !116)
!116 = !{!21, !111, !117}
!117 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !118, size: 64, align: 64, dwarfAddressSpace: 0)
!118 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !31, file: !2, size: 512, align: 64, elements: !119, templateParams: !4, identifier: "1f573d63f2ab1ce613b58742b128b808")
!119 = !{!120, !122, !124, !125, !142, !143}
!120 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !118, file: !2, baseType: !121, size: 32, align: 32, offset: 384)
!121 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!122 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !118, file: !2, baseType: !123, size: 32, align: 32, offset: 416)
!123 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_unsigned_char)
!124 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !118, file: !2, baseType: !28, size: 8, align: 8, offset: 448)
!125 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !118, file: !2, baseType: !126, size: 128, align: 64)
!126 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !127, file: !2, size: 128, align: 64, elements: !128, identifier: "1e8fc19a501ea863e72c3f1e15e073dc")
!127 = !DINamespace(name: "option", scope: !23)
!128 = !{!129}
!129 = !DICompositeType(tag: DW_TAG_variant_part, scope: !127, file: !2, size: 128, align: 64, elements: !130, templateParams: !133, identifier: "1e8fc19a501ea863e72c3f1e15e073dc_variant_part", discriminator: !140)
!130 = !{!131, !136}
!131 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !129, file: !2, baseType: !132, size: 128, align: 64, extraData: i64 0)
!132 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !126, file: !2, size: 128, align: 64, elements: !4, templateParams: !133, identifier: "1e8fc19a501ea863e72c3f1e15e073dc::None")
!133 = !{!134}
!134 = !DITemplateTypeParameter(name: "T", type: !135)
!135 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!136 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !129, file: !2, baseType: !137, size: 128, align: 64, extraData: i64 1)
!137 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !126, file: !2, size: 128, align: 64, elements: !138, templateParams: !133, identifier: "1e8fc19a501ea863e72c3f1e15e073dc::Some")
!138 = !{!139}
!139 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !137, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!140 = !DIDerivedType(tag: DW_TAG_member, scope: !127, file: !2, baseType: !141, size: 64, align: 64, flags: DIFlagArtificial)
!141 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!142 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !118, file: !2, baseType: !126, size: 128, align: 64, offset: 128)
!143 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !118, file: !2, baseType: !144, size: 128, align: 64, offset: 256)
!144 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut Write", scope: !31, file: !2, size: 128, align: 64, elements: !145, templateParams: !4, identifier: "1aafa9c98565f0c76762b1c7dea049cd")
!145 = !{!146, !148}
!146 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !144, file: !2, baseType: !147, size: 64, align: 64, flags: DIFlagArtificial)
!147 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 0)
!148 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !144, file: !2, baseType: !149, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!149 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !150, size: 64, align: 64, dwarfAddressSpace: 0)
!150 = !DICompositeType(tag: DW_TAG_array_type, baseType: !135, size: 192, align: 64, elements: !151)
!151 = !{!152}
!152 = !DISubrange(count: 3, lowerBound: 0)
!153 = !DISubroutineType(types: !154)
!154 = !{!108, !155, !157}
!155 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&f32", baseType: !156, size: 64, align: 64, dwarfAddressSpace: 0)
!156 = !DIBasicType(name: "f32", size: 32, encoding: DW_ATE_float)
!157 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&f32, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !158, size: 64, align: 64, dwarfAddressSpace: 0)
!158 = !DISubroutineType(types: !159)
!159 = !{!21, !155, !117}
!160 = !{!161, !162}
!161 = !DILocalVariable(name: "x", arg: 1, scope: !106, file: !107, line: 267, type: !155)
!162 = !DILocalVariable(name: "f", arg: 2, scope: !106, file: !107, line: 267, type: !157)
!163 = !{!164}
!164 = !DITemplateTypeParameter(name: "T", type: !156)
!165 = !DILocation(line: 267, column: 23, scope: !106)
!166 = !DILocation(line: 267, column: 33, scope: !106)
!167 = !DILocation(line: 276, column: 42, scope: !106)
!168 = !DILocation(line: 276, column: 68, scope: !106)
!169 = !DILocation(line: 276, column: 18, scope: !106)
!170 = !DILocation(line: 277, column: 6, scope: !106)
!171 = distinct !DISubprogram(name: "new_v1", linkageName: "_ZN4core3fmt9Arguments6new_v117h20fccb459b893337E", scope: !172, file: !107, line: 313, type: !234, scopeLine: 313, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !4, retainedNodes: !236)
!172 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !31, file: !2, size: 384, align: 64, elements: !173, templateParams: !4, identifier: "76b7a9bb3aed71d786837bddfb0753ec")
!173 = !{!174, !184, !228}
!174 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !172, file: !2, baseType: !175, size: 128, align: 64)
!175 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !176, templateParams: !4, identifier: "e5181a2ba73cefd2b9372dc5646453a9")
!176 = !{!177, !183}
!177 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !175, file: !2, baseType: !178, size: 64, align: 64)
!178 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const &str", baseType: !179, size: 64, align: 64, dwarfAddressSpace: 0)
!179 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !180, templateParams: !4, identifier: "7ef2a91eecc7bcf4b4aaea2dbce79437")
!180 = !{!181, !182}
!181 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !179, file: !2, baseType: !65, size: 64, align: 64)
!182 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !179, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!183 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !175, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!184 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !172, file: !2, baseType: !185, size: 128, align: 64, offset: 128)
!185 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::v1::Argument]>", scope: !127, file: !2, size: 128, align: 64, elements: !186, identifier: "aaae938ffef63c21303efdefe6cb2dbb")
!186 = !{!187}
!187 = !DICompositeType(tag: DW_TAG_variant_part, scope: !127, file: !2, size: 128, align: 64, elements: !188, templateParams: !191, identifier: "aaae938ffef63c21303efdefe6cb2dbb_variant_part", discriminator: !140)
!188 = !{!189, !224}
!189 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !187, file: !2, baseType: !190, size: 128, align: 64, extraData: i64 0)
!190 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !185, file: !2, size: 128, align: 64, elements: !4, templateParams: !191, identifier: "aaae938ffef63c21303efdefe6cb2dbb::None")
!191 = !{!192}
!192 = !DITemplateTypeParameter(name: "T", type: !193)
!193 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::v1::Argument]", file: !2, size: 128, align: 64, elements: !194, templateParams: !4, identifier: "74d515130a6fe79f849222b698698b47")
!194 = !{!195, !223}
!195 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !193, file: !2, baseType: !196, size: 64, align: 64)
!196 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::rt::v1::Argument", baseType: !197, size: 64, align: 64, dwarfAddressSpace: 0)
!197 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !29, file: !2, size: 448, align: 64, elements: !198, templateParams: !4, identifier: "f08f75da3c994a39573138ccfbe8ac60")
!198 = !{!199, !200}
!199 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !197, file: !2, baseType: !135, size: 64, align: 64)
!200 = !DIDerivedType(tag: DW_TAG_member, name: "format", scope: !197, file: !2, baseType: !201, size: 384, align: 64, offset: 64)
!201 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormatSpec", scope: !29, file: !2, size: 384, align: 64, elements: !202, templateParams: !4, identifier: "b378376481d6c5daa3b3c0ea89d23753")
!202 = !{!203, !204, !205, !206, !222}
!203 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !201, file: !2, baseType: !123, size: 32, align: 32, offset: 256)
!204 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !201, file: !2, baseType: !28, size: 8, align: 8, offset: 320)
!205 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !201, file: !2, baseType: !121, size: 32, align: 32, offset: 288)
!206 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !201, file: !2, baseType: !207, size: 128, align: 64)
!207 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !29, file: !2, size: 128, align: 64, elements: !208, identifier: "5aebf68034f1dabb67782dd4f8e7d101")
!208 = !{!209}
!209 = !DICompositeType(tag: DW_TAG_variant_part, scope: !29, file: !2, size: 128, align: 64, elements: !210, templateParams: !4, identifier: "5aebf68034f1dabb67782dd4f8e7d101_variant_part", discriminator: !221)
!210 = !{!211, !215, !219}
!211 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !209, file: !2, baseType: !212, size: 128, align: 64, extraData: i64 0)
!212 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !207, file: !2, size: 128, align: 64, elements: !213, templateParams: !4, identifier: "5aebf68034f1dabb67782dd4f8e7d101::Is")
!213 = !{!214}
!214 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !212, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!215 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !209, file: !2, baseType: !216, size: 128, align: 64, extraData: i64 1)
!216 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !207, file: !2, size: 128, align: 64, elements: !217, templateParams: !4, identifier: "5aebf68034f1dabb67782dd4f8e7d101::Param")
!217 = !{!218}
!218 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !216, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!219 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !209, file: !2, baseType: !220, size: 128, align: 64, extraData: i64 2)
!220 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !207, file: !2, size: 128, align: 64, elements: !4, templateParams: !4, identifier: "5aebf68034f1dabb67782dd4f8e7d101::Implied")
!221 = !DIDerivedType(tag: DW_TAG_member, scope: !29, file: !2, baseType: !141, size: 64, align: 64, flags: DIFlagArtificial)
!222 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !201, file: !2, baseType: !207, size: 128, align: 64, offset: 128)
!223 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !193, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!224 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !187, file: !2, baseType: !225, size: 128, align: 64)
!225 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !185, file: !2, size: 128, align: 64, elements: !226, templateParams: !191, identifier: "aaae938ffef63c21303efdefe6cb2dbb::Some")
!226 = !{!227}
!227 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !225, file: !2, baseType: !193, size: 128, align: 64)
!228 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !172, file: !2, baseType: !229, size: 128, align: 64, offset: 256)
!229 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::ArgumentV1]", file: !2, size: 128, align: 64, elements: !230, templateParams: !4, identifier: "66438e16d71ef5a1e10493d50a03212")
!230 = !{!231, !233}
!231 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !229, file: !2, baseType: !232, size: 64, align: 64)
!232 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::ArgumentV1", baseType: !108, size: 64, align: 64, dwarfAddressSpace: 0)
!233 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !229, file: !2, baseType: !135, size: 64, align: 64, offset: 64)
!234 = !DISubroutineType(types: !235)
!235 = !{!172, !175, !229}
!236 = !{!237, !238}
!237 = !DILocalVariable(name: "pieces", arg: 1, scope: !171, file: !107, line: 313, type: !175)
!238 = !DILocalVariable(name: "args", arg: 2, scope: !171, file: !107, line: 313, type: !229)
!239 = !DILocation(line: 313, column: 19, scope: !171)
!240 = !DILocation(line: 313, column: 47, scope: !171)
!241 = !DILocation(line: 314, column: 34, scope: !171)
!242 = !DILocation(line: 314, column: 9, scope: !171)
!243 = !DILocation(line: 315, column: 6, scope: !171)
!244 = distinct !DISubprogram(name: "call_once<closure-0,()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9c2a928dfb99cf6bE", scope: !246, file: !245, line: 227, type: !249, scopeLine: 227, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !255, retainedNodes: !252)
!245 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "e7b2206724943b8a8140f7c1065997a3")
!246 = !DINamespace(name: "FnOnce", scope: !247)
!247 = !DINamespace(name: "function", scope: !248)
!248 = !DINamespace(name: "ops", scope: !23)
!249 = !DISubroutineType(types: !250)
!250 = !{!81, !251}
!251 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut closure-0", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 0)
!252 = !{!253, !254}
!253 = !DILocalVariable(arg: 1, scope: !244, file: !245, line: 227, type: !251)
!254 = !DILocalVariable(arg: 2, scope: !244, file: !245, line: 227, type: !48)
!255 = !{!256, !257}
!256 = !DITemplateTypeParameter(name: "Self", type: !5)
!257 = !DITemplateTypeParameter(name: "Args", type: !48)
!258 = !DILocation(line: 227, column: 5, scope: !244)
!259 = distinct !DISubprogram(name: "call_once<fn(),()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h03752571149a459dE", scope: !246, file: !245, line: 227, type: !42, scopeLine: 227, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !263, retainedNodes: !260)
!260 = !{!261, !262}
!261 = !DILocalVariable(arg: 1, scope: !259, file: !245, line: 227, type: !11)
!262 = !DILocalVariable(arg: 2, scope: !259, file: !245, line: 227, type: !48)
!263 = !{!264, !257}
!264 = !DITemplateTypeParameter(name: "Self", type: !11)
!265 = !DILocation(line: 227, column: 5, scope: !259)
!266 = distinct !DISubprogram(name: "call_once<closure-0,()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17he03076b2f3e69f12E", scope: !246, file: !245, line: 227, type: !267, scopeLine: 227, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !255, retainedNodes: !269)
!267 = !DISubroutineType(types: !268)
!268 = !{!81, !5}
!269 = !{!270, !271}
!270 = !DILocalVariable(arg: 1, scope: !266, file: !245, line: 227, type: !5)
!271 = !DILocalVariable(arg: 2, scope: !266, file: !245, line: 227, type: !48)
!272 = !DILocation(line: 227, column: 5, scope: !266)
!273 = distinct !DISubprogram(name: "drop_in_place<closure-0>", linkageName: "_ZN4core3ptr13drop_in_place17h08086275706499c8E", scope: !275, file: !274, line: 179, type: !276, scopeLine: 179, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !280, retainedNodes: !278)
!274 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "5bd0a17517ec02b2d4f77a2ea68243bd")
!275 = !DINamespace(name: "ptr", scope: !23)
!276 = !DISubroutineType(types: !277)
!277 = !{null, !251}
!278 = !{!279}
!279 = !DILocalVariable(arg: 1, scope: !273, file: !274, line: 179, type: !251)
!280 = !{!281}
!281 = !DITemplateTypeParameter(name: "T", type: !5)
!282 = !DILocation(line: 179, column: 1, scope: !273)
!283 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17h1b76f0af3b5a2a39E", scope: !285, file: !284, line: 159, type: !286, scopeLine: 159, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !70, retainedNodes: !288)
!284 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "7d98876a74ab56d91ea54c537ec9a84f")
!285 = !DINamespace(name: "hint", scope: !23)
!286 = !DISubroutineType(types: !287)
!287 = !{null, !48}
!288 = !{!289}
!289 = !DILocalVariable(name: "dummy", arg: 1, scope: !283, file: !284, line: 159, type: !48)
!290 = !DILocation(line: 159, column: 21, scope: !283)
!291 = !DILocation(line: 170, column: 9, scope: !283)
!292 = !{i32 2696673}
!293 = !DILocation(line: 174, column: 2, scope: !283)
!294 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97a7ffb8f66dfce9E", scope: !296, file: !295, line: 1828, type: !298, scopeLine: 1828, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !4, retainedNodes: !300)
!295 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "af3deb4955b6b6f458d03bf15fd16325")
!296 = !DINamespace(name: "{{impl}}", scope: !297)
!297 = !DINamespace(name: "process", scope: !8)
!298 = !DISubroutineType(types: !299)
!299 = !{!81, !48}
!300 = !{!301}
!301 = !DILocalVariable(name: "self", arg: 1, scope: !294, file: !295, line: 1828, type: !48)
!302 = !DILocation(line: 1828, column: 15, scope: !294)
!303 = !DILocation(line: 1829, column: 9, scope: !294)
!304 = !DILocation(line: 1830, column: 6, scope: !294)
!305 = distinct !DISubprogram(name: "report", linkageName: "_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h217f181c02ce20e5E", scope: !296, file: !295, line: 1862, type: !306, scopeLine: 1862, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !4, retainedNodes: !311)
!306 = !DISubroutineType(types: !307)
!307 = !{!81, !308}
!308 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !297, file: !2, size: 8, align: 8, elements: !309, templateParams: !4, identifier: "da622ed335d47ab191c7f0216490d248")
!309 = !{!310}
!310 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !308, file: !2, baseType: !91, size: 8, align: 8)
!311 = !{!312}
!312 = !DILocalVariable(name: "self", arg: 1, scope: !305, file: !295, line: 1862, type: !308)
!313 = !DILocation(line: 1862, column: 15, scope: !305)
!314 = !DILocation(line: 1863, column: 9, scope: !305)
!315 = !DILocation(line: 1864, column: 6, scope: !305)
!316 = distinct !DISubprogram(name: "main", linkageName: "_ZN4main4main17h2d6c3d678af9e020E", scope: !318, file: !317, line: 1, type: !12, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !18, templateParams: !4, retainedNodes: !319)
!317 = !DIFile(filename: "../example/src/main.rs", directory: "/Users/tiberio/Documents/github/oxide-c-api/enzyme", checksumkind: CSK_MD5, checksum: "0596f13e25df0ed5d3c95cc45d4c2e7e")
!318 = !DINamespace(name: "main", scope: null)
!319 = !{!320, !322}
!320 = !DILocalVariable(name: "result", scope: !321, file: !317, line: 2, type: !156, align: 4)
!321 = distinct !DILexicalBlock(scope: !316, file: !317, line: 2, column: 5)
!322 = !DILocalVariable(name: "arg0", scope: !323, file: !317, line: 3, type: !155, align: 8)
!323 = !DILexicalBlockFile(scope: !324, file: !317, discriminator: 0)
!324 = distinct !DILexicalBlock(scope: !321, file: !325, line: 96, column: 28)
!325 = !DIFile(filename: "/Users/tiberio/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "1c225e8c9b34a0d06ab2c182e477c2fe")
!326 = !DILocation(line: 2, column: 9, scope: !321)
!327 = !DILocation(line: 2, column: 34, scope: !316)
!328 = !DILocation(line: 2, column: 43, scope: !316)
!329 = !DILocation(line: 2, column: 18, scope: !316)
!330 = !DILocation(line: 3, column: 5, scope: !321)
!331 = !DILocation(line: 3, column: 5, scope: !323)
!332 = !DILocation(line: 4, column: 2, scope: !316)
!333 = distinct !DISubprogram(name: "simple_mul", linkageName: "_ZN4main10simple_mul17he69584a08c5f72d6E", scope: !318, file: !317, line: 6, type: !334, scopeLine: 6, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !18, templateParams: !4, retainedNodes: !337)
!334 = !DISubroutineType(types: !335)
!335 = !{!156, !336, !336}
!336 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut f32", baseType: !156, size: 64, align: 64, dwarfAddressSpace: 0)
!337 = !{!338, !339}
!338 = !DILocalVariable(name: "left", arg: 1, scope: !333, file: !317, line: 6, type: !336)
!339 = !DILocalVariable(name: "right", arg: 2, scope: !333, file: !317, line: 6, type: !336)
!340 = !DILocation(line: 6, column: 15, scope: !333)
!341 = !DILocation(line: 6, column: 31, scope: !333)
!342 = !DILocation(line: 7, column: 5, scope: !333)
!343 = !DILocation(line: 7, column: 13, scope: !333)
!344 = !DILocation(line: 8, column: 2, scope: !333)

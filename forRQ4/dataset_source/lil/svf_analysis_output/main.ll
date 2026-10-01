; ModuleID = 'main.c'
source_filename = "main.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct._IO_FILE = type { i32, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, %struct._IO_marker*, %struct._IO_FILE*, i32, i32, i64, i16, i8, [1 x i8], i8*, i64, %struct._IO_codecvt*, %struct._IO_wide_data*, %struct._IO_FILE*, i8*, i64, i32, [20 x i8] }
%struct._IO_marker = type opaque
%struct._IO_codecvt = type opaque
%struct._IO_wide_data = type opaque
%struct._lil_t = type opaque
%struct._lil_value_t = type opaque
%struct._lil_list_t = type opaque
%struct._lil_var_t = type opaque

@running = internal global i32 1, align 4, !dbg !0
@exit_code = internal global i32 0, align 4, !dbg !18
@.str = private unnamed_addr constant [2 x i8] c"r\00", align 1
@.str.1 = private unnamed_addr constant [3 x i8] c"%c\00", align 1
@stdin = external global %struct._IO_FILE*, align 8
@.str.2 = private unnamed_addr constant [10 x i8] c"writechar\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"system\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"readline\00", align 1
@.str.5 = private unnamed_addr constant [47 x i8] c"Little Interpreted Language Interactive Shell\0A\00", align 1
@.str.6 = private unnamed_addr constant [3 x i8] c"# \00", align 1
@.str.7 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str.8 = private unnamed_addr constant [17 x i8] c"error at %i: %s\0A\00", align 1
@.str.9 = private unnamed_addr constant [5 x i8] c"argv\00", align 1
@.str.10 = private unnamed_addr constant [155 x i8] c"set __lilmain:code__ [read {%s}]\0Aif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\0A\00", align 1
@stderr = external global %struct._IO_FILE*, align 8
@.str.11 = private unnamed_addr constant [22 x i8] c"lil: error at %i: %s\0A\00", align 1

; Function Attrs: noinline nounwind uwtable
define weak zeroext i16 @__bswap_16(i16 noundef zeroext %__bsx) #0 !dbg !28 {
entry:
  %__bsx.addr = alloca i16, align 2
  store i16 %__bsx, i16* %__bsx.addr, align 2
  call void @llvm.dbg.declare(metadata i16* %__bsx.addr, metadata !33, metadata !DIExpression()), !dbg !34
  %0 = load i16, i16* %__bsx.addr, align 2, !dbg !35
  %conv = zext i16 %0 to i32, !dbg !35
  %shr = ashr i32 %conv, 8, !dbg !35
  %and = and i32 %shr, 255, !dbg !35
  %1 = load i16, i16* %__bsx.addr, align 2, !dbg !35
  %conv1 = zext i16 %1 to i32, !dbg !35
  %and2 = and i32 %conv1, 255, !dbg !35
  %shl = shl i32 %and2, 8, !dbg !35
  %or = or i32 %and, %shl, !dbg !35
  %conv3 = trunc i32 %or to i16, !dbg !35
  ret i16 %conv3, !dbg !36
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: noinline nounwind uwtable
define weak i32 @__bswap_32(i32 noundef %__bsx) #0 !dbg !37 {
entry:
  %__bsx.addr = alloca i32, align 4
  store i32 %__bsx, i32* %__bsx.addr, align 4
  call void @llvm.dbg.declare(metadata i32* %__bsx.addr, metadata !42, metadata !DIExpression()), !dbg !43
  %0 = load i32, i32* %__bsx.addr, align 4, !dbg !44
  %and = and i32 %0, -16777216, !dbg !44
  %shr = lshr i32 %and, 24, !dbg !44
  %1 = load i32, i32* %__bsx.addr, align 4, !dbg !44
  %and1 = and i32 %1, 16711680, !dbg !44
  %shr2 = lshr i32 %and1, 8, !dbg !44
  %or = or i32 %shr, %shr2, !dbg !44
  %2 = load i32, i32* %__bsx.addr, align 4, !dbg !44
  %and3 = and i32 %2, 65280, !dbg !44
  %shl = shl i32 %and3, 8, !dbg !44
  %or4 = or i32 %or, %shl, !dbg !44
  %3 = load i32, i32* %__bsx.addr, align 4, !dbg !44
  %and5 = and i32 %3, 255, !dbg !44
  %shl6 = shl i32 %and5, 24, !dbg !44
  %or7 = or i32 %or4, %shl6, !dbg !44
  ret i32 %or7, !dbg !45
}

; Function Attrs: noinline nounwind uwtable
define weak i64 @__bswap_64(i64 noundef %__bsx) #0 !dbg !46 {
entry:
  %__bsx.addr = alloca i64, align 8
  store i64 %__bsx, i64* %__bsx.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %__bsx.addr, metadata !51, metadata !DIExpression()), !dbg !52
  %0 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and = and i64 %0, -72057594037927936, !dbg !53
  %shr = lshr i64 %and, 56, !dbg !53
  %1 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and1 = and i64 %1, 71776119061217280, !dbg !53
  %shr2 = lshr i64 %and1, 40, !dbg !53
  %or = or i64 %shr, %shr2, !dbg !53
  %2 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and3 = and i64 %2, 280375465082880, !dbg !53
  %shr4 = lshr i64 %and3, 24, !dbg !53
  %or5 = or i64 %or, %shr4, !dbg !53
  %3 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and6 = and i64 %3, 1095216660480, !dbg !53
  %shr7 = lshr i64 %and6, 8, !dbg !53
  %or8 = or i64 %or5, %shr7, !dbg !53
  %4 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and9 = and i64 %4, 4278190080, !dbg !53
  %shl = shl i64 %and9, 8, !dbg !53
  %or10 = or i64 %or8, %shl, !dbg !53
  %5 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and11 = and i64 %5, 16711680, !dbg !53
  %shl12 = shl i64 %and11, 24, !dbg !53
  %or13 = or i64 %or10, %shl12, !dbg !53
  %6 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and14 = and i64 %6, 65280, !dbg !53
  %shl15 = shl i64 %and14, 40, !dbg !53
  %or16 = or i64 %or13, %shl15, !dbg !53
  %7 = load i64, i64* %__bsx.addr, align 8, !dbg !53
  %and17 = and i64 %7, 255, !dbg !53
  %shl18 = shl i64 %and17, 56, !dbg !53
  %or19 = or i64 %or16, %shl18, !dbg !53
  ret i64 %or19, !dbg !54
}

; Function Attrs: noinline nounwind uwtable
define weak zeroext i16 @__uint16_identity(i16 noundef zeroext %__x) #0 !dbg !55 {
entry:
  %__x.addr = alloca i16, align 2
  store i16 %__x, i16* %__x.addr, align 2
  call void @llvm.dbg.declare(metadata i16* %__x.addr, metadata !57, metadata !DIExpression()), !dbg !58
  %0 = load i16, i16* %__x.addr, align 2, !dbg !59
  ret i16 %0, !dbg !60
}

; Function Attrs: noinline nounwind uwtable
define weak i32 @__uint32_identity(i32 noundef %__x) #0 !dbg !61 {
entry:
  %__x.addr = alloca i32, align 4
  store i32 %__x, i32* %__x.addr, align 4
  call void @llvm.dbg.declare(metadata i32* %__x.addr, metadata !62, metadata !DIExpression()), !dbg !63
  %0 = load i32, i32* %__x.addr, align 4, !dbg !64
  ret i32 %0, !dbg !65
}

; Function Attrs: noinline nounwind uwtable
define weak i64 @__uint64_identity(i64 noundef %__x) #0 !dbg !66 {
entry:
  %__x.addr = alloca i64, align 8
  store i64 %__x, i64* %__x.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %__x.addr, metadata !67, metadata !DIExpression()), !dbg !68
  %0 = load i64, i64* %__x.addr, align 8, !dbg !69
  ret i64 %0, !dbg !70
}

; Function Attrs: noinline nounwind uwtable
define weak void @do_exit(%struct._lil_t* noundef %lil, %struct._lil_value_t* noundef %val) #0 !dbg !71 {
entry:
  %lil.addr = alloca %struct._lil_t*, align 8
  %val.addr = alloca %struct._lil_value_t*, align 8
  store %struct._lil_t* %lil, %struct._lil_t** %lil.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil.addr, metadata !80, metadata !DIExpression()), !dbg !81
  store %struct._lil_value_t* %val, %struct._lil_value_t** %val.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %val.addr, metadata !82, metadata !DIExpression()), !dbg !83
  store i32 0, i32* @running, align 4, !dbg !84
  %0 = load %struct._lil_value_t*, %struct._lil_value_t** %val.addr, align 8, !dbg !85
  %call = call i64 @lil_to_integer(%struct._lil_value_t* noundef %0), !dbg !86
  %conv = trunc i64 %call to i32, !dbg !87
  store i32 %conv, i32* @exit_code, align 4, !dbg !88
  ret void, !dbg !89
}

declare i64 @lil_to_integer(%struct._lil_value_t* noundef) #2

; Function Attrs: noinline nounwind uwtable
define weak i8* @do_system(i64 noundef %argc, i8** noundef %argv) #0 !dbg !90 {
entry:
  %retval = alloca i8*, align 8
  %argc.addr = alloca i64, align 8
  %argv.addr = alloca i8**, align 8
  %cmd = alloca i8*, align 8
  %cmdlen = alloca i32, align 4
  %i = alloca i64, align 8
  %p = alloca %struct._IO_FILE*, align 8
  %len = alloca i64, align 8
  %retval19 = alloca i8*, align 8
  %size = alloca i64, align 8
  %buff = alloca [1024 x i8], align 16
  %bytes = alloca i64, align 8
  store i64 %argc, i64* %argc.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.addr, metadata !95, metadata !DIExpression()), !dbg !96
  store i8** %argv, i8*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.addr, metadata !97, metadata !DIExpression()), !dbg !98
  call void @llvm.dbg.declare(metadata i8** %cmd, metadata !99, metadata !DIExpression()), !dbg !100
  store i8* null, i8** %cmd, align 8, !dbg !100
  call void @llvm.dbg.declare(metadata i32* %cmdlen, metadata !101, metadata !DIExpression()), !dbg !102
  store i32 0, i32* %cmdlen, align 4, !dbg !102
  call void @llvm.dbg.declare(metadata i64* %i, metadata !103, metadata !DIExpression()), !dbg !104
  call void @llvm.dbg.declare(metadata %struct._IO_FILE** %p, metadata !105, metadata !DIExpression()), !dbg !161
  store i64 0, i64* %i, align 8, !dbg !162
  br label %for.cond, !dbg !164

for.cond:                                         ; preds = %for.inc, %entry
  %0 = load i64, i64* %i, align 8, !dbg !165
  %1 = load i64, i64* %argc.addr, align 8, !dbg !167
  %cmp = icmp ult i64 %0, %1, !dbg !168
  br i1 %cmp, label %for.body, label %for.end, !dbg !169

for.body:                                         ; preds = %for.cond
  call void @llvm.dbg.declare(metadata i64* %len, metadata !170, metadata !DIExpression()), !dbg !172
  %2 = load i8**, i8*** %argv.addr, align 8, !dbg !173
  %3 = load i64, i64* %i, align 8, !dbg !174
  %arrayidx = getelementptr inbounds i8*, i8** %2, i64 %3, !dbg !173
  %4 = load i8*, i8** %arrayidx, align 8, !dbg !173
  %call = call i64 @strlen(i8* noundef %4) #6, !dbg !175
  store i64 %call, i64* %len, align 8, !dbg !172
  %5 = load i64, i64* %i, align 8, !dbg !176
  %cmp1 = icmp ne i64 %5, 0, !dbg !178
  br i1 %cmp1, label %if.then, label %if.end, !dbg !179

if.then:                                          ; preds = %for.body
  %6 = load i8*, i8** %cmd, align 8, !dbg !180
  %7 = load i32, i32* %cmdlen, align 4, !dbg !182
  %add = add nsw i32 %7, 1, !dbg !183
  %conv = sext i32 %add to i64, !dbg !182
  %call2 = call i8* @realloc(i8* noundef %6, i64 noundef %conv) #7, !dbg !184
  store i8* %call2, i8** %cmd, align 8, !dbg !185
  %8 = load i8*, i8** %cmd, align 8, !dbg !186
  %9 = load i32, i32* %cmdlen, align 4, !dbg !187
  %inc = add nsw i32 %9, 1, !dbg !187
  store i32 %inc, i32* %cmdlen, align 4, !dbg !187
  %idxprom = sext i32 %9 to i64, !dbg !186
  %arrayidx3 = getelementptr inbounds i8, i8* %8, i64 %idxprom, !dbg !186
  store i8 32, i8* %arrayidx3, align 1, !dbg !188
  br label %if.end, !dbg !189

if.end:                                           ; preds = %if.then, %for.body
  %10 = load i8*, i8** %cmd, align 8, !dbg !190
  %11 = load i32, i32* %cmdlen, align 4, !dbg !191
  %conv4 = sext i32 %11 to i64, !dbg !191
  %12 = load i64, i64* %len, align 8, !dbg !192
  %add5 = add i64 %conv4, %12, !dbg !193
  %call6 = call i8* @realloc(i8* noundef %10, i64 noundef %add5) #7, !dbg !194
  store i8* %call6, i8** %cmd, align 8, !dbg !195
  %13 = load i8*, i8** %cmd, align 8, !dbg !196
  %14 = load i32, i32* %cmdlen, align 4, !dbg !197
  %idx.ext = sext i32 %14 to i64, !dbg !198
  %add.ptr = getelementptr inbounds i8, i8* %13, i64 %idx.ext, !dbg !198
  %15 = load i8**, i8*** %argv.addr, align 8, !dbg !199
  %16 = load i64, i64* %i, align 8, !dbg !200
  %arrayidx7 = getelementptr inbounds i8*, i8** %15, i64 %16, !dbg !199
  %17 = load i8*, i8** %arrayidx7, align 8, !dbg !199
  %18 = load i64, i64* %len, align 8, !dbg !201
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %add.ptr, i8* align 1 %17, i64 %18, i1 false), !dbg !202
  %19 = load i64, i64* %len, align 8, !dbg !203
  %20 = load i32, i32* %cmdlen, align 4, !dbg !204
  %conv8 = sext i32 %20 to i64, !dbg !204
  %add9 = add i64 %conv8, %19, !dbg !204
  %conv10 = trunc i64 %add9 to i32, !dbg !204
  store i32 %conv10, i32* %cmdlen, align 4, !dbg !204
  br label %for.inc, !dbg !205

for.inc:                                          ; preds = %if.end
  %21 = load i64, i64* %i, align 8, !dbg !206
  %inc11 = add i64 %21, 1, !dbg !206
  store i64 %inc11, i64* %i, align 8, !dbg !206
  br label %for.cond, !dbg !207, !llvm.loop !208

for.end:                                          ; preds = %for.cond
  %22 = load i8*, i8** %cmd, align 8, !dbg !211
  %23 = load i32, i32* %cmdlen, align 4, !dbg !212
  %add12 = add nsw i32 %23, 1, !dbg !213
  %conv13 = sext i32 %add12 to i64, !dbg !212
  %call14 = call i8* @realloc(i8* noundef %22, i64 noundef %conv13) #7, !dbg !214
  store i8* %call14, i8** %cmd, align 8, !dbg !215
  %24 = load i8*, i8** %cmd, align 8, !dbg !216
  %25 = load i32, i32* %cmdlen, align 4, !dbg !217
  %idxprom15 = sext i32 %25 to i64, !dbg !216
  %arrayidx16 = getelementptr inbounds i8, i8* %24, i64 %idxprom15, !dbg !216
  store i8 0, i8* %arrayidx16, align 1, !dbg !218
  %26 = load i8*, i8** %cmd, align 8, !dbg !219
  %call17 = call %struct._IO_FILE* @popen(i8* noundef %26, i8* noundef getelementptr inbounds ([2 x i8], [2 x i8]* @.str, i64 0, i64 0)), !dbg !220
  store %struct._IO_FILE* %call17, %struct._IO_FILE** %p, align 8, !dbg !221
  %27 = load i8*, i8** %cmd, align 8, !dbg !222
  call void @free(i8* noundef %27) #7, !dbg !223
  %28 = load %struct._IO_FILE*, %struct._IO_FILE** %p, align 8, !dbg !224
  %tobool = icmp ne %struct._IO_FILE* %28, null, !dbg !224
  br i1 %tobool, label %if.then18, label %if.else, !dbg !226

if.then18:                                        ; preds = %for.end
  call void @llvm.dbg.declare(metadata i8** %retval19, metadata !227, metadata !DIExpression()), !dbg !229
  store i8* null, i8** %retval19, align 8, !dbg !229
  call void @llvm.dbg.declare(metadata i64* %size, metadata !230, metadata !DIExpression()), !dbg !231
  store i64 0, i64* %size, align 8, !dbg !231
  call void @llvm.dbg.declare(metadata [1024 x i8]* %buff, metadata !232, metadata !DIExpression()), !dbg !236
  call void @llvm.dbg.declare(metadata i64* %bytes, metadata !237, metadata !DIExpression()), !dbg !241
  br label %while.cond, !dbg !242

while.cond:                                       ; preds = %while.body, %if.then18
  %arraydecay = getelementptr inbounds [1024 x i8], [1024 x i8]* %buff, i64 0, i64 0, !dbg !243
  %29 = load %struct._IO_FILE*, %struct._IO_FILE** %p, align 8, !dbg !244
  %call20 = call i64 @fread(i8* noundef %arraydecay, i64 noundef 1, i64 noundef 1024, %struct._IO_FILE* noundef %29), !dbg !245
  store i64 %call20, i64* %bytes, align 8, !dbg !246
  %tobool21 = icmp ne i64 %call20, 0, !dbg !242
  br i1 %tobool21, label %while.body, label %while.end, !dbg !242

while.body:                                       ; preds = %while.cond
  %30 = load i8*, i8** %retval19, align 8, !dbg !247
  %31 = load i64, i64* %size, align 8, !dbg !249
  %32 = load i64, i64* %bytes, align 8, !dbg !250
  %add22 = add i64 %31, %32, !dbg !251
  %call23 = call i8* @realloc(i8* noundef %30, i64 noundef %add22) #7, !dbg !252
  store i8* %call23, i8** %retval19, align 8, !dbg !253
  %33 = load i8*, i8** %retval19, align 8, !dbg !254
  %34 = load i64, i64* %size, align 8, !dbg !255
  %add.ptr24 = getelementptr inbounds i8, i8* %33, i64 %34, !dbg !256
  %arraydecay25 = getelementptr inbounds [1024 x i8], [1024 x i8]* %buff, i64 0, i64 0, !dbg !257
  %35 = load i64, i64* %bytes, align 8, !dbg !258
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %add.ptr24, i8* align 16 %arraydecay25, i64 %35, i1 false), !dbg !257
  %36 = load i64, i64* %bytes, align 8, !dbg !259
  %37 = load i64, i64* %size, align 8, !dbg !260
  %add26 = add i64 %37, %36, !dbg !260
  store i64 %add26, i64* %size, align 8, !dbg !260
  br label %while.cond, !dbg !242, !llvm.loop !261

while.end:                                        ; preds = %while.cond
  %38 = load i8*, i8** %retval19, align 8, !dbg !263
  %39 = load i64, i64* %size, align 8, !dbg !264
  %add27 = add i64 %39, 1, !dbg !265
  %call28 = call i8* @realloc(i8* noundef %38, i64 noundef %add27) #7, !dbg !266
  store i8* %call28, i8** %retval19, align 8, !dbg !267
  %40 = load i8*, i8** %retval19, align 8, !dbg !268
  %41 = load i64, i64* %size, align 8, !dbg !269
  %arrayidx29 = getelementptr inbounds i8, i8* %40, i64 %41, !dbg !268
  store i8 0, i8* %arrayidx29, align 1, !dbg !270
  %42 = load %struct._IO_FILE*, %struct._IO_FILE** %p, align 8, !dbg !271
  %call30 = call i32 @pclose(%struct._IO_FILE* noundef %42), !dbg !272
  %43 = load i8*, i8** %retval19, align 8, !dbg !273
  store i8* %43, i8** %retval, align 8, !dbg !274
  br label %return, !dbg !274

if.else:                                          ; preds = %for.end
  store i8* null, i8** %retval, align 8, !dbg !275
  br label %return, !dbg !275

return:                                           ; preds = %if.else, %while.end
  %44 = load i8*, i8** %retval, align 8, !dbg !277
  ret i8* %44, !dbg !277
}

; Function Attrs: nounwind readonly willreturn
declare i64 @strlen(i8* noundef) #3

; Function Attrs: nounwind
declare i8* @realloc(i8* noundef, i64 noundef) #4

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #5

declare %struct._IO_FILE* @popen(i8* noundef, i8* noundef) #2

; Function Attrs: nounwind
declare void @free(i8* noundef) #4

declare i64 @fread(i8* noundef, i64 noundef, i64 noundef, %struct._IO_FILE* noundef) #2

declare i32 @pclose(%struct._IO_FILE* noundef) #2

; Function Attrs: noinline nounwind uwtable
define weak %struct._lil_value_t* @fnc_writechar(%struct._lil_t* noundef %lil, i64 noundef %argc, %struct._lil_value_t** noundef %argv) #0 !dbg !278 {
entry:
  %retval = alloca %struct._lil_value_t*, align 8
  %lil.addr = alloca %struct._lil_t*, align 8
  %argc.addr = alloca i64, align 8
  %argv.addr = alloca %struct._lil_value_t**, align 8
  store %struct._lil_t* %lil, %struct._lil_t** %lil.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil.addr, metadata !282, metadata !DIExpression()), !dbg !283
  store i64 %argc, i64* %argc.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.addr, metadata !284, metadata !DIExpression()), !dbg !285
  store %struct._lil_value_t** %argv, %struct._lil_value_t*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_value_t*** %argv.addr, metadata !286, metadata !DIExpression()), !dbg !287
  %0 = load i64, i64* %argc.addr, align 8, !dbg !288
  %tobool = icmp ne i64 %0, 0, !dbg !288
  br i1 %tobool, label %if.end, label %if.then, !dbg !290

if.then:                                          ; preds = %entry
  store %struct._lil_value_t* null, %struct._lil_value_t** %retval, align 8, !dbg !291
  br label %return, !dbg !291

if.end:                                           ; preds = %entry
  %1 = load %struct._lil_value_t**, %struct._lil_value_t*** %argv.addr, align 8, !dbg !292
  %arrayidx = getelementptr inbounds %struct._lil_value_t*, %struct._lil_value_t** %1, i64 0, !dbg !292
  %2 = load %struct._lil_value_t*, %struct._lil_value_t** %arrayidx, align 8, !dbg !292
  %call = call i64 @lil_to_integer(%struct._lil_value_t* noundef %2), !dbg !293
  %conv = trunc i64 %call to i8, !dbg !294
  %conv1 = sext i8 %conv to i32, !dbg !294
  %call2 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([3 x i8], [3 x i8]* @.str.1, i64 0, i64 0), i32 noundef %conv1), !dbg !295
  store %struct._lil_value_t* null, %struct._lil_value_t** %retval, align 8, !dbg !296
  br label %return, !dbg !296

return:                                           ; preds = %if.end, %if.then
  %3 = load %struct._lil_value_t*, %struct._lil_value_t** %retval, align 8, !dbg !297
  ret %struct._lil_value_t* %3, !dbg !297
}

declare i32 @printf(i8* noundef, ...) #2

; Function Attrs: noinline nounwind uwtable
define weak %struct._lil_value_t* @fnc_system(%struct._lil_t* noundef %lil, i64 noundef %argc, %struct._lil_value_t** noundef %argv) #0 !dbg !298 {
entry:
  %retval = alloca %struct._lil_value_t*, align 8
  %lil.addr = alloca %struct._lil_t*, align 8
  %argc.addr = alloca i64, align 8
  %argv.addr = alloca %struct._lil_value_t**, align 8
  %sargv = alloca i8**, align 8
  %r = alloca %struct._lil_value_t*, align 8
  %rv = alloca i8*, align 8
  %i = alloca i64, align 8
  store %struct._lil_t* %lil, %struct._lil_t** %lil.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil.addr, metadata !299, metadata !DIExpression()), !dbg !300
  store i64 %argc, i64* %argc.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.addr, metadata !301, metadata !DIExpression()), !dbg !302
  store %struct._lil_value_t** %argv, %struct._lil_value_t*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_value_t*** %argv.addr, metadata !303, metadata !DIExpression()), !dbg !304
  call void @llvm.dbg.declare(metadata i8*** %sargv, metadata !305, metadata !DIExpression()), !dbg !309
  %0 = load i64, i64* %argc.addr, align 8, !dbg !310
  %add = add i64 %0, 1, !dbg !311
  %mul = mul i64 8, %add, !dbg !312
  %call = call noalias i8* @malloc(i64 noundef %mul) #7, !dbg !313
  %1 = bitcast i8* %call to i8**, !dbg !313
  store i8** %1, i8*** %sargv, align 8, !dbg !309
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %r, metadata !314, metadata !DIExpression()), !dbg !315
  store %struct._lil_value_t* null, %struct._lil_value_t** %r, align 8, !dbg !315
  call void @llvm.dbg.declare(metadata i8** %rv, metadata !316, metadata !DIExpression()), !dbg !317
  call void @llvm.dbg.declare(metadata i64* %i, metadata !318, metadata !DIExpression()), !dbg !319
  %2 = load i64, i64* %argc.addr, align 8, !dbg !320
  %cmp = icmp eq i64 %2, 0, !dbg !322
  br i1 %cmp, label %if.then, label %if.end, !dbg !323

if.then:                                          ; preds = %entry
  store %struct._lil_value_t* null, %struct._lil_value_t** %retval, align 8, !dbg !324
  br label %return, !dbg !324

if.end:                                           ; preds = %entry
  store i64 0, i64* %i, align 8, !dbg !325
  br label %for.cond, !dbg !327

for.cond:                                         ; preds = %for.inc, %if.end
  %3 = load i64, i64* %i, align 8, !dbg !328
  %4 = load i64, i64* %argc.addr, align 8, !dbg !330
  %cmp1 = icmp ult i64 %3, %4, !dbg !331
  br i1 %cmp1, label %for.body, label %for.end, !dbg !332

for.body:                                         ; preds = %for.cond
  %5 = load %struct._lil_value_t**, %struct._lil_value_t*** %argv.addr, align 8, !dbg !333
  %6 = load i64, i64* %i, align 8, !dbg !334
  %arrayidx = getelementptr inbounds %struct._lil_value_t*, %struct._lil_value_t** %5, i64 %6, !dbg !333
  %7 = load %struct._lil_value_t*, %struct._lil_value_t** %arrayidx, align 8, !dbg !333
  %call2 = call i8* @lil_to_string(%struct._lil_value_t* noundef %7), !dbg !335
  %8 = load i8**, i8*** %sargv, align 8, !dbg !336
  %9 = load i64, i64* %i, align 8, !dbg !337
  %arrayidx3 = getelementptr inbounds i8*, i8** %8, i64 %9, !dbg !336
  store i8* %call2, i8** %arrayidx3, align 8, !dbg !338
  br label %for.inc, !dbg !336

for.inc:                                          ; preds = %for.body
  %10 = load i64, i64* %i, align 8, !dbg !339
  %inc = add i64 %10, 1, !dbg !339
  store i64 %inc, i64* %i, align 8, !dbg !339
  br label %for.cond, !dbg !340, !llvm.loop !341

for.end:                                          ; preds = %for.cond
  %11 = load i8**, i8*** %sargv, align 8, !dbg !343
  %12 = load i64, i64* %argc.addr, align 8, !dbg !344
  %arrayidx4 = getelementptr inbounds i8*, i8** %11, i64 %12, !dbg !343
  store i8* null, i8** %arrayidx4, align 8, !dbg !345
  %13 = load i64, i64* %argc.addr, align 8, !dbg !346
  %14 = load i8**, i8*** %sargv, align 8, !dbg !347
  %call5 = call i8* @do_system(i64 noundef %13, i8** noundef %14), !dbg !348
  store i8* %call5, i8** %rv, align 8, !dbg !349
  %15 = load i8*, i8** %rv, align 8, !dbg !350
  %tobool = icmp ne i8* %15, null, !dbg !350
  br i1 %tobool, label %if.then6, label %if.end8, !dbg !352

if.then6:                                         ; preds = %for.end
  %16 = load i8*, i8** %rv, align 8, !dbg !353
  %call7 = call %struct._lil_value_t* @lil_alloc_string(i8* noundef %16), !dbg !355
  store %struct._lil_value_t* %call7, %struct._lil_value_t** %r, align 8, !dbg !356
  %17 = load i8*, i8** %rv, align 8, !dbg !357
  call void @free(i8* noundef %17) #7, !dbg !358
  br label %if.end8, !dbg !359

if.end8:                                          ; preds = %if.then6, %for.end
  %18 = load i8**, i8*** %sargv, align 8, !dbg !360
  %19 = bitcast i8** %18 to i8*, !dbg !360
  call void @free(i8* noundef %19) #7, !dbg !361
  %20 = load %struct._lil_value_t*, %struct._lil_value_t** %r, align 8, !dbg !362
  store %struct._lil_value_t* %20, %struct._lil_value_t** %retval, align 8, !dbg !363
  br label %return, !dbg !363

return:                                           ; preds = %if.end8, %if.then
  %21 = load %struct._lil_value_t*, %struct._lil_value_t** %retval, align 8, !dbg !364
  ret %struct._lil_value_t* %21, !dbg !364
}

; Function Attrs: nounwind
declare noalias i8* @malloc(i64 noundef) #4

declare i8* @lil_to_string(%struct._lil_value_t* noundef) #2

declare %struct._lil_value_t* @lil_alloc_string(i8* noundef) #2

; Function Attrs: noinline nounwind uwtable
define weak %struct._lil_value_t* @fnc_readline(%struct._lil_t* noundef %lil, i64 noundef %argc, %struct._lil_value_t** noundef %argv) #0 !dbg !365 {
entry:
  %lil.addr = alloca %struct._lil_t*, align 8
  %argc.addr = alloca i64, align 8
  %argv.addr = alloca %struct._lil_value_t**, align 8
  %len = alloca i64, align 8
  %size = alloca i64, align 8
  %buffer = alloca i8*, align 8
  %ch = alloca i8, align 1
  %retval1 = alloca %struct._lil_value_t*, align 8
  store %struct._lil_t* %lil, %struct._lil_t** %lil.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil.addr, metadata !366, metadata !DIExpression()), !dbg !367
  store i64 %argc, i64* %argc.addr, align 8
  call void @llvm.dbg.declare(metadata i64* %argc.addr, metadata !368, metadata !DIExpression()), !dbg !369
  store %struct._lil_value_t** %argv, %struct._lil_value_t*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata %struct._lil_value_t*** %argv.addr, metadata !370, metadata !DIExpression()), !dbg !371
  call void @llvm.dbg.declare(metadata i64* %len, metadata !372, metadata !DIExpression()), !dbg !373
  store i64 0, i64* %len, align 8, !dbg !373
  call void @llvm.dbg.declare(metadata i64* %size, metadata !374, metadata !DIExpression()), !dbg !375
  store i64 64, i64* %size, align 8, !dbg !375
  call void @llvm.dbg.declare(metadata i8** %buffer, metadata !376, metadata !DIExpression()), !dbg !377
  %0 = load i64, i64* %size, align 8, !dbg !378
  %call = call noalias i8* @malloc(i64 noundef %0) #7, !dbg !379
  store i8* %call, i8** %buffer, align 8, !dbg !377
  call void @llvm.dbg.declare(metadata i8* %ch, metadata !380, metadata !DIExpression()), !dbg !381
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %retval1, metadata !382, metadata !DIExpression()), !dbg !383
  br label %for.cond, !dbg !384

for.cond:                                         ; preds = %if.end19, %if.then8, %entry
  %1 = load %struct._IO_FILE*, %struct._IO_FILE** @stdin, align 8, !dbg !385
  %call2 = call i32 @fgetc(%struct._IO_FILE* noundef %1), !dbg !389
  %conv = trunc i32 %call2 to i8, !dbg !389
  store i8 %conv, i8* %ch, align 1, !dbg !390
  %2 = load i8, i8* %ch, align 1, !dbg !391
  %conv3 = sext i8 %2 to i32, !dbg !391
  %cmp = icmp eq i32 %conv3, -1, !dbg !393
  br i1 %cmp, label %if.then, label %if.end, !dbg !394

if.then:                                          ; preds = %for.cond
  br label %for.end, !dbg !395

if.end:                                           ; preds = %for.cond
  %3 = load i8, i8* %ch, align 1, !dbg !396
  %conv5 = sext i8 %3 to i32, !dbg !396
  %cmp6 = icmp eq i32 %conv5, 13, !dbg !398
  br i1 %cmp6, label %if.then8, label %if.end9, !dbg !399

if.then8:                                         ; preds = %if.end
  br label %for.cond, !dbg !400, !llvm.loop !401

if.end9:                                          ; preds = %if.end
  %4 = load i8, i8* %ch, align 1, !dbg !404
  %conv10 = sext i8 %4 to i32, !dbg !404
  %cmp11 = icmp eq i32 %conv10, 10, !dbg !406
  br i1 %cmp11, label %if.then13, label %if.end14, !dbg !407

if.then13:                                        ; preds = %if.end9
  br label %for.end, !dbg !408

if.end14:                                         ; preds = %if.end9
  %5 = load i64, i64* %len, align 8, !dbg !409
  %6 = load i64, i64* %size, align 8, !dbg !411
  %cmp15 = icmp ult i64 %5, %6, !dbg !412
  br i1 %cmp15, label %if.then17, label %if.end19, !dbg !413

if.then17:                                        ; preds = %if.end14
  %7 = load i64, i64* %size, align 8, !dbg !414
  %add = add i64 %7, 64, !dbg !414
  store i64 %add, i64* %size, align 8, !dbg !414
  %8 = load i8*, i8** %buffer, align 8, !dbg !416
  %9 = load i64, i64* %size, align 8, !dbg !417
  %call18 = call i8* @realloc(i8* noundef %8, i64 noundef %9) #7, !dbg !418
  store i8* %call18, i8** %buffer, align 8, !dbg !419
  br label %if.end19, !dbg !420

if.end19:                                         ; preds = %if.then17, %if.end14
  %10 = load i8, i8* %ch, align 1, !dbg !421
  %11 = load i8*, i8** %buffer, align 8, !dbg !422
  %12 = load i64, i64* %len, align 8, !dbg !423
  %inc = add i64 %12, 1, !dbg !423
  store i64 %inc, i64* %len, align 8, !dbg !423
  %arrayidx = getelementptr inbounds i8, i8* %11, i64 %12, !dbg !422
  store i8 %10, i8* %arrayidx, align 1, !dbg !424
  br label %for.cond, !dbg !425, !llvm.loop !401

for.end:                                          ; preds = %if.then13, %if.then
  %13 = load i8*, i8** %buffer, align 8, !dbg !426
  %14 = load i64, i64* %len, align 8, !dbg !427
  %add20 = add i64 %14, 1, !dbg !428
  %call21 = call i8* @realloc(i8* noundef %13, i64 noundef %add20) #7, !dbg !429
  store i8* %call21, i8** %buffer, align 8, !dbg !430
  %15 = load i8*, i8** %buffer, align 8, !dbg !431
  %16 = load i64, i64* %len, align 8, !dbg !432
  %arrayidx22 = getelementptr inbounds i8, i8* %15, i64 %16, !dbg !431
  store i8 0, i8* %arrayidx22, align 1, !dbg !433
  %17 = load i8*, i8** %buffer, align 8, !dbg !434
  %call23 = call %struct._lil_value_t* @lil_alloc_string(i8* noundef %17), !dbg !435
  store %struct._lil_value_t* %call23, %struct._lil_value_t** %retval1, align 8, !dbg !436
  %18 = load i8*, i8** %buffer, align 8, !dbg !437
  call void @free(i8* noundef %18) #7, !dbg !438
  %19 = load %struct._lil_value_t*, %struct._lil_value_t** %retval1, align 8, !dbg !439
  ret %struct._lil_value_t* %19, !dbg !440
}

declare i32 @fgetc(%struct._IO_FILE* noundef) #2

; Function Attrs: noinline nounwind uwtable
define weak i32 @repl() #0 !dbg !441 {
entry:
  %buffer = alloca [16384 x i8], align 16
  %lil = alloca %struct._lil_t*, align 8
  %result = alloca %struct._lil_value_t*, align 8
  %strres = alloca i8*, align 8
  %err_msg = alloca i8*, align 8
  %pos = alloca i64, align 8
  call void @llvm.dbg.declare(metadata [16384 x i8]* %buffer, metadata !444, metadata !DIExpression()), !dbg !448
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil, metadata !449, metadata !DIExpression()), !dbg !450
  %call = call %struct._lil_t* @lil_new(), !dbg !451
  store %struct._lil_t* %call, %struct._lil_t** %lil, align 8, !dbg !450
  %0 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !452
  %call1 = call i32 @lil_register(%struct._lil_t* noundef %0, i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.2, i64 0, i64 0), %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef @fnc_writechar), !dbg !453
  %1 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !454
  %call2 = call i32 @lil_register(%struct._lil_t* noundef %1, i8* noundef getelementptr inbounds ([7 x i8], [7 x i8]* @.str.3, i64 0, i64 0), %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef @fnc_system), !dbg !455
  %2 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !456
  %call3 = call i32 @lil_register(%struct._lil_t* noundef %2, i8* noundef getelementptr inbounds ([9 x i8], [9 x i8]* @.str.4, i64 0, i64 0), %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef @fnc_readline), !dbg !457
  %call4 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([47 x i8], [47 x i8]* @.str.5, i64 0, i64 0)), !dbg !458
  %3 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !459
  call void @lil_callback(%struct._lil_t* noundef %3, i32 noundef 0, void ()* noundef bitcast (void (%struct._lil_t*, %struct._lil_value_t*)* @do_exit to void ()*)), !dbg !460
  br label %while.cond, !dbg !461

while.cond:                                       ; preds = %if.end20, %entry
  %4 = load i32, i32* @running, align 4, !dbg !462
  %tobool = icmp ne i32 %4, 0, !dbg !461
  br i1 %tobool, label %while.body, label %while.end, !dbg !461

while.body:                                       ; preds = %while.cond
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %result, metadata !463, metadata !DIExpression()), !dbg !465
  call void @llvm.dbg.declare(metadata i8** %strres, metadata !466, metadata !DIExpression()), !dbg !467
  call void @llvm.dbg.declare(metadata i8** %err_msg, metadata !468, metadata !DIExpression()), !dbg !469
  call void @llvm.dbg.declare(metadata i64* %pos, metadata !470, metadata !DIExpression()), !dbg !471
  %arrayidx = getelementptr inbounds [16384 x i8], [16384 x i8]* %buffer, i64 0, i64 0, !dbg !472
  store i8 0, i8* %arrayidx, align 16, !dbg !473
  %call5 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([3 x i8], [3 x i8]* @.str.6, i64 0, i64 0)), !dbg !474
  %arraydecay = getelementptr inbounds [16384 x i8], [16384 x i8]* %buffer, i64 0, i64 0, !dbg !475
  %5 = load %struct._IO_FILE*, %struct._IO_FILE** @stdin, align 8, !dbg !477
  %call6 = call i8* @fgets(i8* noundef %arraydecay, i32 noundef 16384, %struct._IO_FILE* noundef %5), !dbg !478
  %tobool7 = icmp ne i8* %call6, null, !dbg !478
  br i1 %tobool7, label %if.end, label %if.then, !dbg !479

if.then:                                          ; preds = %while.body
  br label %while.end, !dbg !480

if.end:                                           ; preds = %while.body
  %6 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !481
  %arraydecay8 = getelementptr inbounds [16384 x i8], [16384 x i8]* %buffer, i64 0, i64 0, !dbg !482
  %call9 = call %struct._lil_value_t* @lil_parse(%struct._lil_t* noundef %6, i8* noundef %arraydecay8, i64 noundef 0, i32 noundef 0), !dbg !483
  store %struct._lil_value_t* %call9, %struct._lil_value_t** %result, align 8, !dbg !484
  %7 = load %struct._lil_value_t*, %struct._lil_value_t** %result, align 8, !dbg !485
  %call10 = call i8* @lil_to_string(%struct._lil_value_t* noundef %7), !dbg !486
  store i8* %call10, i8** %strres, align 8, !dbg !487
  %8 = load i8*, i8** %strres, align 8, !dbg !488
  %arrayidx11 = getelementptr inbounds i8, i8* %8, i64 0, !dbg !488
  %9 = load i8, i8* %arrayidx11, align 1, !dbg !488
  %tobool12 = icmp ne i8 %9, 0, !dbg !488
  br i1 %tobool12, label %if.then13, label %if.end15, !dbg !490

if.then13:                                        ; preds = %if.end
  %10 = load i8*, i8** %strres, align 8, !dbg !491
  %call14 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.7, i64 0, i64 0), i8* noundef %10), !dbg !492
  br label %if.end15, !dbg !492

if.end15:                                         ; preds = %if.then13, %if.end
  %11 = load %struct._lil_value_t*, %struct._lil_value_t** %result, align 8, !dbg !493
  call void @lil_free_value(%struct._lil_value_t* noundef %11), !dbg !494
  %12 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !495
  %call16 = call i32 @lil_error(%struct._lil_t* noundef %12, i8** noundef %err_msg, i64* noundef %pos), !dbg !497
  %tobool17 = icmp ne i32 %call16, 0, !dbg !497
  br i1 %tobool17, label %if.then18, label %if.end20, !dbg !498

if.then18:                                        ; preds = %if.end15
  %13 = load i64, i64* %pos, align 8, !dbg !499
  %conv = trunc i64 %13 to i32, !dbg !501
  %14 = load i8*, i8** %err_msg, align 8, !dbg !502
  %call19 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([17 x i8], [17 x i8]* @.str.8, i64 0, i64 0), i32 noundef %conv, i8* noundef %14), !dbg !503
  br label %if.end20, !dbg !504

if.end20:                                         ; preds = %if.then18, %if.end15
  br label %while.cond, !dbg !461, !llvm.loop !505

while.end:                                        ; preds = %if.then, %while.cond
  %15 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !507
  call void @lil_free(%struct._lil_t* noundef %15), !dbg !508
  %16 = load i32, i32* @exit_code, align 4, !dbg !509
  ret i32 %16, !dbg !510
}

declare %struct._lil_t* @lil_new() #2

declare i32 @lil_register(%struct._lil_t* noundef, i8* noundef, %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef) #2

declare void @lil_callback(%struct._lil_t* noundef, i32 noundef, void ()* noundef) #2

declare i8* @fgets(i8* noundef, i32 noundef, %struct._IO_FILE* noundef) #2

declare %struct._lil_value_t* @lil_parse(%struct._lil_t* noundef, i8* noundef, i64 noundef, i32 noundef) #2

declare void @lil_free_value(%struct._lil_value_t* noundef) #2

declare i32 @lil_error(%struct._lil_t* noundef, i8** noundef, i64* noundef) #2

declare void @lil_free(%struct._lil_t* noundef) #2

; Function Attrs: noinline nounwind uwtable
define weak i32 @nonint(i32 noundef %argc, i8** noundef %argv) #0 !dbg !511 {
entry:
  %argc.addr = alloca i32, align 4
  %argv.addr = alloca i8**, align 8
  %lil = alloca %struct._lil_t*, align 8
  %filename = alloca i8*, align 8
  %err_msg = alloca i8*, align 8
  %pos = alloca i64, align 8
  %arglist = alloca %struct._lil_list_t*, align 8
  %args = alloca %struct._lil_value_t*, align 8
  %result = alloca %struct._lil_value_t*, align 8
  %tmpcode = alloca i8*, align 8
  %i = alloca i32, align 4
  store i32 %argc, i32* %argc.addr, align 4
  call void @llvm.dbg.declare(metadata i32* %argc.addr, metadata !514, metadata !DIExpression()), !dbg !515
  store i8** %argv, i8*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.addr, metadata !516, metadata !DIExpression()), !dbg !517
  call void @llvm.dbg.declare(metadata %struct._lil_t** %lil, metadata !518, metadata !DIExpression()), !dbg !519
  %call = call %struct._lil_t* @lil_new(), !dbg !520
  store %struct._lil_t* %call, %struct._lil_t** %lil, align 8, !dbg !519
  call void @llvm.dbg.declare(metadata i8** %filename, metadata !521, metadata !DIExpression()), !dbg !522
  %0 = load i8**, i8*** %argv.addr, align 8, !dbg !523
  %arrayidx = getelementptr inbounds i8*, i8** %0, i64 1, !dbg !523
  %1 = load i8*, i8** %arrayidx, align 8, !dbg !523
  store i8* %1, i8** %filename, align 8, !dbg !522
  call void @llvm.dbg.declare(metadata i8** %err_msg, metadata !524, metadata !DIExpression()), !dbg !525
  call void @llvm.dbg.declare(metadata i64* %pos, metadata !526, metadata !DIExpression()), !dbg !527
  call void @llvm.dbg.declare(metadata %struct._lil_list_t** %arglist, metadata !528, metadata !DIExpression()), !dbg !532
  %call1 = call %struct._lil_list_t* @lil_alloc_list(), !dbg !533
  store %struct._lil_list_t* %call1, %struct._lil_list_t** %arglist, align 8, !dbg !532
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %args, metadata !534, metadata !DIExpression()), !dbg !535
  call void @llvm.dbg.declare(metadata %struct._lil_value_t** %result, metadata !536, metadata !DIExpression()), !dbg !537
  call void @llvm.dbg.declare(metadata i8** %tmpcode, metadata !538, metadata !DIExpression()), !dbg !539
  call void @llvm.dbg.declare(metadata i32* %i, metadata !540, metadata !DIExpression()), !dbg !541
  %2 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !542
  %call2 = call i32 @lil_register(%struct._lil_t* noundef %2, i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.2, i64 0, i64 0), %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef @fnc_writechar), !dbg !543
  %3 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !544
  %call3 = call i32 @lil_register(%struct._lil_t* noundef %3, i8* noundef getelementptr inbounds ([7 x i8], [7 x i8]* @.str.3, i64 0, i64 0), %struct._lil_value_t* (%struct._lil_t*, i64, %struct._lil_value_t**)* noundef @fnc_system), !dbg !545
  store i32 2, i32* %i, align 4, !dbg !546
  br label %for.cond, !dbg !548

for.cond:                                         ; preds = %for.inc, %entry
  %4 = load i32, i32* %i, align 4, !dbg !549
  %5 = load i32, i32* %argc.addr, align 4, !dbg !551
  %cmp = icmp slt i32 %4, %5, !dbg !552
  br i1 %cmp, label %for.body, label %for.end, !dbg !553

for.body:                                         ; preds = %for.cond
  %6 = load %struct._lil_list_t*, %struct._lil_list_t** %arglist, align 8, !dbg !554
  %7 = load i8**, i8*** %argv.addr, align 8, !dbg !556
  %8 = load i32, i32* %i, align 4, !dbg !557
  %idxprom = sext i32 %8 to i64, !dbg !556
  %arrayidx4 = getelementptr inbounds i8*, i8** %7, i64 %idxprom, !dbg !556
  %9 = load i8*, i8** %arrayidx4, align 8, !dbg !556
  %call5 = call %struct._lil_value_t* @lil_alloc_string(i8* noundef %9), !dbg !558
  call void @lil_list_append(%struct._lil_list_t* noundef %6, %struct._lil_value_t* noundef %call5), !dbg !559
  br label %for.inc, !dbg !560

for.inc:                                          ; preds = %for.body
  %10 = load i32, i32* %i, align 4, !dbg !561
  %inc = add nsw i32 %10, 1, !dbg !561
  store i32 %inc, i32* %i, align 4, !dbg !561
  br label %for.cond, !dbg !562, !llvm.loop !563

for.end:                                          ; preds = %for.cond
  %11 = load %struct._lil_list_t*, %struct._lil_list_t** %arglist, align 8, !dbg !565
  %call6 = call %struct._lil_value_t* @lil_list_to_value(%struct._lil_list_t* noundef %11, i32 noundef 1), !dbg !566
  store %struct._lil_value_t* %call6, %struct._lil_value_t** %args, align 8, !dbg !567
  %12 = load %struct._lil_list_t*, %struct._lil_list_t** %arglist, align 8, !dbg !568
  call void @lil_free_list(%struct._lil_list_t* noundef %12), !dbg !569
  %13 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !570
  %14 = load %struct._lil_value_t*, %struct._lil_value_t** %args, align 8, !dbg !571
  %call7 = call %struct._lil_var_t* @lil_set_var(%struct._lil_t* noundef %13, i8* noundef getelementptr inbounds ([5 x i8], [5 x i8]* @.str.9, i64 0, i64 0), %struct._lil_value_t* noundef %14, i32 noundef 0), !dbg !572
  %15 = load %struct._lil_value_t*, %struct._lil_value_t** %args, align 8, !dbg !573
  call void @lil_free_value(%struct._lil_value_t* noundef %15), !dbg !574
  %16 = load i8*, i8** %filename, align 8, !dbg !575
  %call8 = call i64 @strlen(i8* noundef %16) #6, !dbg !576
  %add = add i64 %call8, 256, !dbg !577
  %call9 = call noalias i8* @malloc(i64 noundef %add) #7, !dbg !578
  store i8* %call9, i8** %tmpcode, align 8, !dbg !579
  %17 = load i8*, i8** %tmpcode, align 8, !dbg !580
  %18 = load i8*, i8** %filename, align 8, !dbg !581
  %call10 = call i32 (i8*, i8*, ...) @sprintf(i8* noundef %17, i8* noundef getelementptr inbounds ([155 x i8], [155 x i8]* @.str.10, i64 0, i64 0), i8* noundef %18) #7, !dbg !582
  %19 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !583
  %20 = load i8*, i8** %tmpcode, align 8, !dbg !584
  %call11 = call %struct._lil_value_t* @lil_parse(%struct._lil_t* noundef %19, i8* noundef %20, i64 noundef 0, i32 noundef 1), !dbg !585
  store %struct._lil_value_t* %call11, %struct._lil_value_t** %result, align 8, !dbg !586
  %21 = load i8*, i8** %tmpcode, align 8, !dbg !587
  call void @free(i8* noundef %21) #7, !dbg !588
  %22 = load %struct._lil_value_t*, %struct._lil_value_t** %result, align 8, !dbg !589
  call void @lil_free_value(%struct._lil_value_t* noundef %22), !dbg !590
  %23 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !591
  %call12 = call i32 @lil_error(%struct._lil_t* noundef %23, i8** noundef %err_msg, i64* noundef %pos), !dbg !593
  %tobool = icmp ne i32 %call12, 0, !dbg !593
  br i1 %tobool, label %if.then, label %if.end, !dbg !594

if.then:                                          ; preds = %for.end
  %24 = load %struct._IO_FILE*, %struct._IO_FILE** @stderr, align 8, !dbg !595
  %25 = load i64, i64* %pos, align 8, !dbg !597
  %conv = trunc i64 %25 to i32, !dbg !598
  %26 = load i8*, i8** %err_msg, align 8, !dbg !599
  %call13 = call i32 (%struct._IO_FILE*, i8*, ...) @fprintf(%struct._IO_FILE* noundef %24, i8* noundef getelementptr inbounds ([22 x i8], [22 x i8]* @.str.11, i64 0, i64 0), i32 noundef %conv, i8* noundef %26), !dbg !600
  br label %if.end, !dbg !601

if.end:                                           ; preds = %if.then, %for.end
  %27 = load %struct._lil_t*, %struct._lil_t** %lil, align 8, !dbg !602
  call void @lil_free(%struct._lil_t* noundef %27), !dbg !603
  %28 = load i32, i32* @exit_code, align 4, !dbg !604
  ret i32 %28, !dbg !605
}

declare %struct._lil_list_t* @lil_alloc_list() #2

declare void @lil_list_append(%struct._lil_list_t* noundef, %struct._lil_value_t* noundef) #2

declare %struct._lil_value_t* @lil_list_to_value(%struct._lil_list_t* noundef, i32 noundef) #2

declare void @lil_free_list(%struct._lil_list_t* noundef) #2

declare %struct._lil_var_t* @lil_set_var(%struct._lil_t* noundef, i8* noundef, %struct._lil_value_t* noundef, i32 noundef) #2

; Function Attrs: nounwind
declare i32 @sprintf(i8* noundef, i8* noundef, ...) #4

declare i32 @fprintf(%struct._IO_FILE* noundef, i8* noundef, ...) #2

; Function Attrs: noinline nounwind uwtable
define weak i32 @main(i32 noundef %argc, i8** noundef %argv) #0 !dbg !606 {
entry:
  %retval = alloca i32, align 4
  %argc.addr = alloca i32, align 4
  %argv.addr = alloca i8**, align 8
  store i32 0, i32* %retval, align 4
  store i32 %argc, i32* %argc.addr, align 4
  call void @llvm.dbg.declare(metadata i32* %argc.addr, metadata !607, metadata !DIExpression()), !dbg !608
  store i8** %argv, i8*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.addr, metadata !609, metadata !DIExpression()), !dbg !610
  %0 = load i32, i32* %argc.addr, align 4, !dbg !611
  %cmp = icmp slt i32 %0, 2, !dbg !613
  br i1 %cmp, label %if.then, label %if.else, !dbg !614

if.then:                                          ; preds = %entry
  %call = call i32 @repl(), !dbg !615
  store i32 %call, i32* %retval, align 4, !dbg !616
  br label %return, !dbg !616

if.else:                                          ; preds = %entry
  %1 = load i32, i32* %argc.addr, align 4, !dbg !617
  %2 = load i8**, i8*** %argv.addr, align 8, !dbg !618
  %call1 = call i32 @nonint(i32 noundef %1, i8** noundef %2), !dbg !619
  store i32 %call1, i32* %retval, align 4, !dbg !620
  br label %return, !dbg !620

return:                                           ; preds = %if.else, %if.then
  %3 = load i32, i32* %retval, align 4, !dbg !621
  ret i32 %3, !dbg !621
}

attributes #0 = { noinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind readonly willreturn "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { argmemonly nofree nounwind willreturn }
attributes #6 = { nounwind readonly willreturn }
attributes #7 = { nounwind }

!llvm.dbg.cu = !{!2}
!llvm.module.flags = !{!20, !21, !22, !23, !24, !25, !26}
!llvm.ident = !{!27}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "running", scope: !2, file: !3, line: 38, type: !8, isLocal: true, isDefinition: true)
!2 = distinct !DICompileUnit(language: DW_LANG_C99, file: !3, producer: "Ubuntu clang version 14.0.6", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, retainedTypes: !4, globals: !17, splitDebugInlining: false, nameTableKind: None)
!3 = !DIFile(filename: "main.c", directory: "/home/anonymous/artifact/PerfTrans/dataset_source/lil", checksumkind: CSK_MD5, checksum: "b34f1cbb1c5c8c546dcae8e03de1692d")
!4 = !{!5, !8, !9, !10, !12}
!5 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint16_t", file: !6, line: 40, baseType: !7)
!6 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/types.h", directory: "", checksumkind: CSK_MD5, checksum: "f6304b1a6dcfc6bee76e9a51043b5090")
!7 = !DIBasicType(name: "unsigned short", size: 16, encoding: DW_ATE_unsigned)
!8 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!9 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!10 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !11, size: 64)
!11 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !9, size: 64)
!12 = !DIDerivedType(tag: DW_TAG_typedef, name: "lil_callback_proc_t", file: !13, line: 89, baseType: !14)
!13 = !DIFile(filename: "./lil.h", directory: "/home/anonymous/artifact/PerfTrans/dataset_source/lil", checksumkind: CSK_MD5, checksum: "94acb62c91d55010556e0a270bb37085")
!14 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !15, size: 64)
!15 = !DISubroutineType(types: !16)
!16 = !{null}
!17 = !{!0, !18}
!18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
!19 = distinct !DIGlobalVariable(name: "exit_code", scope: !2, file: !3, line: 39, type: !8, isLocal: true, isDefinition: true)
!20 = !{i32 7, !"Dwarf Version", i32 5}
!21 = !{i32 2, !"Debug Info Version", i32 3}
!22 = !{i32 1, !"wchar_size", i32 4}
!23 = !{i32 7, !"PIC Level", i32 2}
!24 = !{i32 7, !"PIE Level", i32 2}
!25 = !{i32 7, !"uwtable", i32 1}
!26 = !{i32 7, !"frame-pointer", i32 2}
!27 = !{!"Ubuntu clang version 14.0.6"}
!28 = distinct !DISubprogram(name: "__bswap_16", scope: !29, file: !29, line: 34, type: !30, scopeLine: 35, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!29 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/byteswap.h", directory: "", checksumkind: CSK_MD5, checksum: "552c402ec2d372531713984b317e0c35")
!30 = !DISubroutineType(types: !31)
!31 = !{!5, !5}
!32 = !{}
!33 = !DILocalVariable(name: "__bsx", arg: 1, scope: !28, file: !29, line: 34, type: !5)
!34 = !DILocation(line: 34, column: 24, scope: !28)
!35 = !DILocation(line: 39, column: 10, scope: !28)
!36 = !DILocation(line: 39, column: 3, scope: !28)
!37 = distinct !DISubprogram(name: "__bswap_32", scope: !29, file: !29, line: 49, type: !38, scopeLine: 50, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!38 = !DISubroutineType(types: !39)
!39 = !{!40, !40}
!40 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint32_t", file: !6, line: 42, baseType: !41)
!41 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!42 = !DILocalVariable(name: "__bsx", arg: 1, scope: !37, file: !29, line: 49, type: !40)
!43 = !DILocation(line: 49, column: 24, scope: !37)
!44 = !DILocation(line: 54, column: 10, scope: !37)
!45 = !DILocation(line: 54, column: 3, scope: !37)
!46 = distinct !DISubprogram(name: "__bswap_64", scope: !29, file: !29, line: 70, type: !47, scopeLine: 71, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!47 = !DISubroutineType(types: !48)
!48 = !{!49, !49}
!49 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint64_t", file: !6, line: 45, baseType: !50)
!50 = !DIBasicType(name: "unsigned long", size: 64, encoding: DW_ATE_unsigned)
!51 = !DILocalVariable(name: "__bsx", arg: 1, scope: !46, file: !29, line: 70, type: !49)
!52 = !DILocation(line: 70, column: 24, scope: !46)
!53 = !DILocation(line: 75, column: 10, scope: !46)
!54 = !DILocation(line: 75, column: 3, scope: !46)
!55 = distinct !DISubprogram(name: "__uint16_identity", scope: !56, file: !56, line: 33, type: !30, scopeLine: 34, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!56 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h", directory: "", checksumkind: CSK_MD5, checksum: "544939bf4e90270e79098eb977a12dcd")
!57 = !DILocalVariable(name: "__x", arg: 1, scope: !55, file: !56, line: 33, type: !5)
!58 = !DILocation(line: 33, column: 31, scope: !55)
!59 = !DILocation(line: 35, column: 10, scope: !55)
!60 = !DILocation(line: 35, column: 3, scope: !55)
!61 = distinct !DISubprogram(name: "__uint32_identity", scope: !56, file: !56, line: 39, type: !38, scopeLine: 40, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!62 = !DILocalVariable(name: "__x", arg: 1, scope: !61, file: !56, line: 39, type: !40)
!63 = !DILocation(line: 39, column: 31, scope: !61)
!64 = !DILocation(line: 41, column: 10, scope: !61)
!65 = !DILocation(line: 41, column: 3, scope: !61)
!66 = distinct !DISubprogram(name: "__uint64_identity", scope: !56, file: !56, line: 45, type: !47, scopeLine: 46, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!67 = !DILocalVariable(name: "__x", arg: 1, scope: !66, file: !56, line: 45, type: !49)
!68 = !DILocation(line: 45, column: 31, scope: !66)
!69 = !DILocation(line: 47, column: 10, scope: !66)
!70 = !DILocation(line: 47, column: 3, scope: !66)
!71 = distinct !DISubprogram(name: "do_exit", scope: !3, file: !3, line: 41, type: !72, scopeLine: 42, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!72 = !DISubroutineType(types: !73)
!73 = !{null, !74, !77}
!74 = !DIDerivedType(tag: DW_TAG_typedef, name: "lil_t", file: !13, line: 79, baseType: !75)
!75 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !76, size: 64)
!76 = !DICompositeType(tag: DW_TAG_structure_type, name: "_lil_t", file: !13, line: 79, flags: DIFlagFwdDecl)
!77 = !DIDerivedType(tag: DW_TAG_typedef, name: "lil_value_t", file: !13, line: 74, baseType: !78)
!78 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !79, size: 64)
!79 = !DICompositeType(tag: DW_TAG_structure_type, name: "_lil_value_t", file: !13, line: 74, flags: DIFlagFwdDecl)
!80 = !DILocalVariable(name: "lil", arg: 1, scope: !71, file: !3, line: 41, type: !74)
!81 = !DILocation(line: 41, column: 39, scope: !71)
!82 = !DILocalVariable(name: "val", arg: 2, scope: !71, file: !3, line: 41, type: !77)
!83 = !DILocation(line: 41, column: 56, scope: !71)
!84 = !DILocation(line: 43, column: 13, scope: !71)
!85 = !DILocation(line: 44, column: 37, scope: !71)
!86 = !DILocation(line: 44, column: 22, scope: !71)
!87 = !DILocation(line: 44, column: 17, scope: !71)
!88 = !DILocation(line: 44, column: 15, scope: !71)
!89 = !DILocation(line: 45, column: 1, scope: !71)
!90 = distinct !DISubprogram(name: "do_system", scope: !3, file: !3, line: 47, type: !91, scopeLine: 48, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!91 = !DISubroutineType(types: !92)
!92 = !{!11, !93, !10}
!93 = !DIDerivedType(tag: DW_TAG_typedef, name: "size_t", file: !94, line: 46, baseType: !50)
!94 = !DIFile(filename: "/usr/lib/llvm-14/lib/clang/14.0.6/include/stddef.h", directory: "", checksumkind: CSK_MD5, checksum: "2499dd2361b915724b073282bea3a7bc")
!95 = !DILocalVariable(name: "argc", arg: 1, scope: !90, file: !3, line: 47, type: !93)
!96 = !DILocation(line: 47, column: 31, scope: !90)
!97 = !DILocalVariable(name: "argv", arg: 2, scope: !90, file: !3, line: 47, type: !10)
!98 = !DILocation(line: 47, column: 44, scope: !90)
!99 = !DILocalVariable(name: "cmd", scope: !90, file: !3, line: 52, type: !11)
!100 = !DILocation(line: 52, column: 11, scope: !90)
!101 = !DILocalVariable(name: "cmdlen", scope: !90, file: !3, line: 53, type: !8)
!102 = !DILocation(line: 53, column: 9, scope: !90)
!103 = !DILocalVariable(name: "i", scope: !90, file: !3, line: 54, type: !93)
!104 = !DILocation(line: 54, column: 12, scope: !90)
!105 = !DILocalVariable(name: "p", scope: !90, file: !3, line: 55, type: !106)
!106 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !107, size: 64)
!107 = !DIDerivedType(tag: DW_TAG_typedef, name: "FILE", file: !108, line: 7, baseType: !109)
!108 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/types/FILE.h", directory: "", checksumkind: CSK_MD5, checksum: "571f9fb6223c42439075fdde11a0de5d")
!109 = distinct !DICompositeType(tag: DW_TAG_structure_type, name: "_IO_FILE", file: !110, line: 49, size: 1728, elements: !111)
!110 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h", directory: "", checksumkind: CSK_MD5, checksum: "f3c970561f3408448ce03a9676ead8f4")
!111 = !{!112, !113, !114, !115, !116, !117, !118, !119, !120, !121, !122, !123, !124, !127, !129, !130, !131, !134, !135, !137, !141, !144, !146, !149, !152, !153, !155, !156, !157}
!112 = !DIDerivedType(tag: DW_TAG_member, name: "_flags", scope: !109, file: !110, line: 51, baseType: !8, size: 32)
!113 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_read_ptr", scope: !109, file: !110, line: 54, baseType: !11, size: 64, offset: 64)
!114 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_read_end", scope: !109, file: !110, line: 55, baseType: !11, size: 64, offset: 128)
!115 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_read_base", scope: !109, file: !110, line: 56, baseType: !11, size: 64, offset: 192)
!116 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_write_base", scope: !109, file: !110, line: 57, baseType: !11, size: 64, offset: 256)
!117 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_write_ptr", scope: !109, file: !110, line: 58, baseType: !11, size: 64, offset: 320)
!118 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_write_end", scope: !109, file: !110, line: 59, baseType: !11, size: 64, offset: 384)
!119 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_buf_base", scope: !109, file: !110, line: 60, baseType: !11, size: 64, offset: 448)
!120 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_buf_end", scope: !109, file: !110, line: 61, baseType: !11, size: 64, offset: 512)
!121 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_save_base", scope: !109, file: !110, line: 64, baseType: !11, size: 64, offset: 576)
!122 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_backup_base", scope: !109, file: !110, line: 65, baseType: !11, size: 64, offset: 640)
!123 = !DIDerivedType(tag: DW_TAG_member, name: "_IO_save_end", scope: !109, file: !110, line: 66, baseType: !11, size: 64, offset: 704)
!124 = !DIDerivedType(tag: DW_TAG_member, name: "_markers", scope: !109, file: !110, line: 68, baseType: !125, size: 64, offset: 768)
!125 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !126, size: 64)
!126 = !DICompositeType(tag: DW_TAG_structure_type, name: "_IO_marker", file: !110, line: 36, flags: DIFlagFwdDecl)
!127 = !DIDerivedType(tag: DW_TAG_member, name: "_chain", scope: !109, file: !110, line: 70, baseType: !128, size: 64, offset: 832)
!128 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !109, size: 64)
!129 = !DIDerivedType(tag: DW_TAG_member, name: "_fileno", scope: !109, file: !110, line: 72, baseType: !8, size: 32, offset: 896)
!130 = !DIDerivedType(tag: DW_TAG_member, name: "_flags2", scope: !109, file: !110, line: 73, baseType: !8, size: 32, offset: 928)
!131 = !DIDerivedType(tag: DW_TAG_member, name: "_old_offset", scope: !109, file: !110, line: 74, baseType: !132, size: 64, offset: 960)
!132 = !DIDerivedType(tag: DW_TAG_typedef, name: "__off_t", file: !6, line: 152, baseType: !133)
!133 = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
!134 = !DIDerivedType(tag: DW_TAG_member, name: "_cur_column", scope: !109, file: !110, line: 77, baseType: !7, size: 16, offset: 1024)
!135 = !DIDerivedType(tag: DW_TAG_member, name: "_vtable_offset", scope: !109, file: !110, line: 78, baseType: !136, size: 8, offset: 1040)
!136 = !DIBasicType(name: "signed char", size: 8, encoding: DW_ATE_signed_char)
!137 = !DIDerivedType(tag: DW_TAG_member, name: "_shortbuf", scope: !109, file: !110, line: 79, baseType: !138, size: 8, offset: 1048)
!138 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 8, elements: !139)
!139 = !{!140}
!140 = !DISubrange(count: 1)
!141 = !DIDerivedType(tag: DW_TAG_member, name: "_lock", scope: !109, file: !110, line: 81, baseType: !142, size: 64, offset: 1088)
!142 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !143, size: 64)
!143 = !DIDerivedType(tag: DW_TAG_typedef, name: "_IO_lock_t", file: !110, line: 43, baseType: null)
!144 = !DIDerivedType(tag: DW_TAG_member, name: "_offset", scope: !109, file: !110, line: 89, baseType: !145, size: 64, offset: 1152)
!145 = !DIDerivedType(tag: DW_TAG_typedef, name: "__off64_t", file: !6, line: 153, baseType: !133)
!146 = !DIDerivedType(tag: DW_TAG_member, name: "_codecvt", scope: !109, file: !110, line: 91, baseType: !147, size: 64, offset: 1216)
!147 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !148, size: 64)
!148 = !DICompositeType(tag: DW_TAG_structure_type, name: "_IO_codecvt", file: !110, line: 37, flags: DIFlagFwdDecl)
!149 = !DIDerivedType(tag: DW_TAG_member, name: "_wide_data", scope: !109, file: !110, line: 92, baseType: !150, size: 64, offset: 1280)
!150 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !151, size: 64)
!151 = !DICompositeType(tag: DW_TAG_structure_type, name: "_IO_wide_data", file: !110, line: 38, flags: DIFlagFwdDecl)
!152 = !DIDerivedType(tag: DW_TAG_member, name: "_freeres_list", scope: !109, file: !110, line: 93, baseType: !128, size: 64, offset: 1344)
!153 = !DIDerivedType(tag: DW_TAG_member, name: "_freeres_buf", scope: !109, file: !110, line: 94, baseType: !154, size: 64, offset: 1408)
!154 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: null, size: 64)
!155 = !DIDerivedType(tag: DW_TAG_member, name: "__pad5", scope: !109, file: !110, line: 95, baseType: !93, size: 64, offset: 1472)
!156 = !DIDerivedType(tag: DW_TAG_member, name: "_mode", scope: !109, file: !110, line: 96, baseType: !8, size: 32, offset: 1536)
!157 = !DIDerivedType(tag: DW_TAG_member, name: "_unused2", scope: !109, file: !110, line: 98, baseType: !158, size: 160, offset: 1568)
!158 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 160, elements: !159)
!159 = !{!160}
!160 = !DISubrange(count: 20)
!161 = !DILocation(line: 55, column: 11, scope: !90)
!162 = !DILocation(line: 56, column: 11, scope: !163)
!163 = distinct !DILexicalBlock(scope: !90, file: !3, line: 56, column: 5)
!164 = !DILocation(line: 56, column: 10, scope: !163)
!165 = !DILocation(line: 56, column: 15, scope: !166)
!166 = distinct !DILexicalBlock(scope: !163, file: !3, line: 56, column: 5)
!167 = !DILocation(line: 56, column: 17, scope: !166)
!168 = !DILocation(line: 56, column: 16, scope: !166)
!169 = !DILocation(line: 56, column: 5, scope: !163)
!170 = !DILocalVariable(name: "len", scope: !171, file: !3, line: 57, type: !93)
!171 = distinct !DILexicalBlock(scope: !166, file: !3, line: 56, column: 28)
!172 = !DILocation(line: 57, column: 16, scope: !171)
!173 = !DILocation(line: 57, column: 29, scope: !171)
!174 = !DILocation(line: 57, column: 34, scope: !171)
!175 = !DILocation(line: 57, column: 22, scope: !171)
!176 = !DILocation(line: 58, column: 13, scope: !177)
!177 = distinct !DILexicalBlock(scope: !171, file: !3, line: 58, column: 13)
!178 = !DILocation(line: 58, column: 15, scope: !177)
!179 = !DILocation(line: 58, column: 13, scope: !171)
!180 = !DILocation(line: 59, column: 27, scope: !181)
!181 = distinct !DILexicalBlock(scope: !177, file: !3, line: 58, column: 21)
!182 = !DILocation(line: 59, column: 32, scope: !181)
!183 = !DILocation(line: 59, column: 39, scope: !181)
!184 = !DILocation(line: 59, column: 19, scope: !181)
!185 = !DILocation(line: 59, column: 17, scope: !181)
!186 = !DILocation(line: 60, column: 13, scope: !181)
!187 = !DILocation(line: 60, column: 23, scope: !181)
!188 = !DILocation(line: 60, column: 27, scope: !181)
!189 = !DILocation(line: 61, column: 9, scope: !181)
!190 = !DILocation(line: 62, column: 23, scope: !171)
!191 = !DILocation(line: 62, column: 28, scope: !171)
!192 = !DILocation(line: 62, column: 37, scope: !171)
!193 = !DILocation(line: 62, column: 35, scope: !171)
!194 = !DILocation(line: 62, column: 15, scope: !171)
!195 = !DILocation(line: 62, column: 13, scope: !171)
!196 = !DILocation(line: 63, column: 16, scope: !171)
!197 = !DILocation(line: 63, column: 22, scope: !171)
!198 = !DILocation(line: 63, column: 20, scope: !171)
!199 = !DILocation(line: 63, column: 30, scope: !171)
!200 = !DILocation(line: 63, column: 35, scope: !171)
!201 = !DILocation(line: 63, column: 39, scope: !171)
!202 = !DILocation(line: 63, column: 9, scope: !171)
!203 = !DILocation(line: 64, column: 19, scope: !171)
!204 = !DILocation(line: 64, column: 16, scope: !171)
!205 = !DILocation(line: 65, column: 5, scope: !171)
!206 = !DILocation(line: 56, column: 24, scope: !166)
!207 = !DILocation(line: 56, column: 5, scope: !166)
!208 = distinct !{!208, !169, !209, !210}
!209 = !DILocation(line: 65, column: 5, scope: !163)
!210 = !{!"llvm.loop.mustprogress"}
!211 = !DILocation(line: 66, column: 19, scope: !90)
!212 = !DILocation(line: 66, column: 24, scope: !90)
!213 = !DILocation(line: 66, column: 31, scope: !90)
!214 = !DILocation(line: 66, column: 11, scope: !90)
!215 = !DILocation(line: 66, column: 9, scope: !90)
!216 = !DILocation(line: 67, column: 5, scope: !90)
!217 = !DILocation(line: 67, column: 9, scope: !90)
!218 = !DILocation(line: 67, column: 17, scope: !90)
!219 = !DILocation(line: 68, column: 15, scope: !90)
!220 = !DILocation(line: 68, column: 9, scope: !90)
!221 = !DILocation(line: 68, column: 7, scope: !90)
!222 = !DILocation(line: 69, column: 10, scope: !90)
!223 = !DILocation(line: 69, column: 5, scope: !90)
!224 = !DILocation(line: 70, column: 9, scope: !225)
!225 = distinct !DILexicalBlock(scope: !90, file: !3, line: 70, column: 9)
!226 = !DILocation(line: 70, column: 9, scope: !90)
!227 = !DILocalVariable(name: "retval", scope: !228, file: !3, line: 71, type: !11)
!228 = distinct !DILexicalBlock(scope: !225, file: !3, line: 70, column: 12)
!229 = !DILocation(line: 71, column: 15, scope: !228)
!230 = !DILocalVariable(name: "size", scope: !228, file: !3, line: 72, type: !93)
!231 = !DILocation(line: 72, column: 16, scope: !228)
!232 = !DILocalVariable(name: "buff", scope: !228, file: !3, line: 73, type: !233)
!233 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 8192, elements: !234)
!234 = !{!235}
!235 = !DISubrange(count: 1024)
!236 = !DILocation(line: 73, column: 14, scope: !228)
!237 = !DILocalVariable(name: "bytes", scope: !228, file: !3, line: 74, type: !238)
!238 = !DIDerivedType(tag: DW_TAG_typedef, name: "ssize_t", file: !239, line: 220, baseType: !240)
!239 = !DIFile(filename: "/usr/include/unistd.h", directory: "", checksumkind: CSK_MD5, checksum: "29bea3f2d65ec3bb874f389bfdfa2266")
!240 = !DIDerivedType(tag: DW_TAG_typedef, name: "__ssize_t", file: !6, line: 193, baseType: !133)
!241 = !DILocation(line: 74, column: 17, scope: !228)
!242 = !DILocation(line: 75, column: 9, scope: !228)
!243 = !DILocation(line: 75, column: 31, scope: !228)
!244 = !DILocation(line: 75, column: 46, scope: !228)
!245 = !DILocation(line: 75, column: 25, scope: !228)
!246 = !DILocation(line: 75, column: 23, scope: !228)
!247 = !DILocation(line: 76, column: 30, scope: !248)
!248 = distinct !DILexicalBlock(scope: !228, file: !3, line: 75, column: 51)
!249 = !DILocation(line: 76, column: 38, scope: !248)
!250 = !DILocation(line: 76, column: 45, scope: !248)
!251 = !DILocation(line: 76, column: 43, scope: !248)
!252 = !DILocation(line: 76, column: 22, scope: !248)
!253 = !DILocation(line: 76, column: 20, scope: !248)
!254 = !DILocation(line: 77, column: 20, scope: !248)
!255 = !DILocation(line: 77, column: 29, scope: !248)
!256 = !DILocation(line: 77, column: 27, scope: !248)
!257 = !DILocation(line: 77, column: 13, scope: !248)
!258 = !DILocation(line: 77, column: 41, scope: !248)
!259 = !DILocation(line: 78, column: 21, scope: !248)
!260 = !DILocation(line: 78, column: 18, scope: !248)
!261 = distinct !{!261, !242, !262, !210}
!262 = !DILocation(line: 79, column: 9, scope: !228)
!263 = !DILocation(line: 80, column: 26, scope: !228)
!264 = !DILocation(line: 80, column: 34, scope: !228)
!265 = !DILocation(line: 80, column: 39, scope: !228)
!266 = !DILocation(line: 80, column: 18, scope: !228)
!267 = !DILocation(line: 80, column: 16, scope: !228)
!268 = !DILocation(line: 81, column: 9, scope: !228)
!269 = !DILocation(line: 81, column: 16, scope: !228)
!270 = !DILocation(line: 81, column: 22, scope: !228)
!271 = !DILocation(line: 82, column: 16, scope: !228)
!272 = !DILocation(line: 82, column: 9, scope: !228)
!273 = !DILocation(line: 83, column: 16, scope: !228)
!274 = !DILocation(line: 83, column: 9, scope: !228)
!275 = !DILocation(line: 85, column: 9, scope: !276)
!276 = distinct !DILexicalBlock(scope: !225, file: !3, line: 84, column: 12)
!277 = !DILocation(line: 88, column: 1, scope: !90)
!278 = distinct !DISubprogram(name: "fnc_writechar", scope: !3, file: !3, line: 90, type: !279, scopeLine: 91, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!279 = !DISubroutineType(types: !280)
!280 = !{!77, !74, !93, !281}
!281 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !77, size: 64)
!282 = !DILocalVariable(name: "lil", arg: 1, scope: !278, file: !3, line: 90, type: !74)
!283 = !DILocation(line: 90, column: 52, scope: !278)
!284 = !DILocalVariable(name: "argc", arg: 2, scope: !278, file: !3, line: 90, type: !93)
!285 = !DILocation(line: 90, column: 64, scope: !278)
!286 = !DILocalVariable(name: "argv", arg: 3, scope: !278, file: !3, line: 90, type: !281)
!287 = !DILocation(line: 90, column: 83, scope: !278)
!288 = !DILocation(line: 92, column: 10, scope: !289)
!289 = distinct !DILexicalBlock(scope: !278, file: !3, line: 92, column: 9)
!290 = !DILocation(line: 92, column: 9, scope: !278)
!291 = !DILocation(line: 92, column: 16, scope: !289)
!292 = !DILocation(line: 93, column: 39, scope: !278)
!293 = !DILocation(line: 93, column: 24, scope: !278)
!294 = !DILocation(line: 93, column: 18, scope: !278)
!295 = !DILocation(line: 93, column: 5, scope: !278)
!296 = !DILocation(line: 94, column: 5, scope: !278)
!297 = !DILocation(line: 95, column: 1, scope: !278)
!298 = distinct !DISubprogram(name: "fnc_system", scope: !3, file: !3, line: 97, type: !279, scopeLine: 98, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!299 = !DILocalVariable(name: "lil", arg: 1, scope: !298, file: !3, line: 97, type: !74)
!300 = !DILocation(line: 97, column: 49, scope: !298)
!301 = !DILocalVariable(name: "argc", arg: 2, scope: !298, file: !3, line: 97, type: !93)
!302 = !DILocation(line: 97, column: 61, scope: !298)
!303 = !DILocalVariable(name: "argv", arg: 3, scope: !298, file: !3, line: 97, type: !281)
!304 = !DILocation(line: 97, column: 80, scope: !298)
!305 = !DILocalVariable(name: "sargv", scope: !298, file: !3, line: 99, type: !306)
!306 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !307, size: 64)
!307 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !308, size: 64)
!308 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
!309 = !DILocation(line: 99, column: 18, scope: !298)
!310 = !DILocation(line: 99, column: 48, scope: !298)
!311 = !DILocation(line: 99, column: 53, scope: !298)
!312 = !DILocation(line: 99, column: 46, scope: !298)
!313 = !DILocation(line: 99, column: 26, scope: !298)
!314 = !DILocalVariable(name: "r", scope: !298, file: !3, line: 100, type: !77)
!315 = !DILocation(line: 100, column: 17, scope: !298)
!316 = !DILocalVariable(name: "rv", scope: !298, file: !3, line: 101, type: !11)
!317 = !DILocation(line: 101, column: 11, scope: !298)
!318 = !DILocalVariable(name: "i", scope: !298, file: !3, line: 102, type: !93)
!319 = !DILocation(line: 102, column: 12, scope: !298)
!320 = !DILocation(line: 103, column: 9, scope: !321)
!321 = distinct !DILexicalBlock(scope: !298, file: !3, line: 103, column: 9)
!322 = !DILocation(line: 103, column: 14, scope: !321)
!323 = !DILocation(line: 103, column: 9, scope: !298)
!324 = !DILocation(line: 103, column: 20, scope: !321)
!325 = !DILocation(line: 104, column: 11, scope: !326)
!326 = distinct !DILexicalBlock(scope: !298, file: !3, line: 104, column: 5)
!327 = !DILocation(line: 104, column: 10, scope: !326)
!328 = !DILocation(line: 104, column: 15, scope: !329)
!329 = distinct !DILexicalBlock(scope: !326, file: !3, line: 104, column: 5)
!330 = !DILocation(line: 104, column: 17, scope: !329)
!331 = !DILocation(line: 104, column: 16, scope: !329)
!332 = !DILocation(line: 104, column: 5, scope: !326)
!333 = !DILocation(line: 105, column: 34, scope: !329)
!334 = !DILocation(line: 105, column: 39, scope: !329)
!335 = !DILocation(line: 105, column: 20, scope: !329)
!336 = !DILocation(line: 105, column: 9, scope: !329)
!337 = !DILocation(line: 105, column: 15, scope: !329)
!338 = !DILocation(line: 105, column: 18, scope: !329)
!339 = !DILocation(line: 104, column: 24, scope: !329)
!340 = !DILocation(line: 104, column: 5, scope: !329)
!341 = distinct !{!341, !332, !342, !210}
!342 = !DILocation(line: 105, column: 41, scope: !326)
!343 = !DILocation(line: 106, column: 5, scope: !298)
!344 = !DILocation(line: 106, column: 11, scope: !298)
!345 = !DILocation(line: 106, column: 17, scope: !298)
!346 = !DILocation(line: 107, column: 20, scope: !298)
!347 = !DILocation(line: 107, column: 34, scope: !298)
!348 = !DILocation(line: 107, column: 10, scope: !298)
!349 = !DILocation(line: 107, column: 8, scope: !298)
!350 = !DILocation(line: 108, column: 9, scope: !351)
!351 = distinct !DILexicalBlock(scope: !298, file: !3, line: 108, column: 9)
!352 = !DILocation(line: 108, column: 9, scope: !298)
!353 = !DILocation(line: 109, column: 30, scope: !354)
!354 = distinct !DILexicalBlock(scope: !351, file: !3, line: 108, column: 13)
!355 = !DILocation(line: 109, column: 13, scope: !354)
!356 = !DILocation(line: 109, column: 11, scope: !354)
!357 = !DILocation(line: 110, column: 14, scope: !354)
!358 = !DILocation(line: 110, column: 9, scope: !354)
!359 = !DILocation(line: 111, column: 5, scope: !354)
!360 = !DILocation(line: 112, column: 10, scope: !298)
!361 = !DILocation(line: 112, column: 5, scope: !298)
!362 = !DILocation(line: 113, column: 12, scope: !298)
!363 = !DILocation(line: 113, column: 5, scope: !298)
!364 = !DILocation(line: 114, column: 1, scope: !298)
!365 = distinct !DISubprogram(name: "fnc_readline", scope: !3, file: !3, line: 116, type: !279, scopeLine: 117, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!366 = !DILocalVariable(name: "lil", arg: 1, scope: !365, file: !3, line: 116, type: !74)
!367 = !DILocation(line: 116, column: 51, scope: !365)
!368 = !DILocalVariable(name: "argc", arg: 2, scope: !365, file: !3, line: 116, type: !93)
!369 = !DILocation(line: 116, column: 63, scope: !365)
!370 = !DILocalVariable(name: "argv", arg: 3, scope: !365, file: !3, line: 116, type: !281)
!371 = !DILocation(line: 116, column: 82, scope: !365)
!372 = !DILocalVariable(name: "len", scope: !365, file: !3, line: 118, type: !93)
!373 = !DILocation(line: 118, column: 12, scope: !365)
!374 = !DILocalVariable(name: "size", scope: !365, file: !3, line: 118, type: !93)
!375 = !DILocation(line: 118, column: 21, scope: !365)
!376 = !DILocalVariable(name: "buffer", scope: !365, file: !3, line: 119, type: !11)
!377 = !DILocation(line: 119, column: 11, scope: !365)
!378 = !DILocation(line: 119, column: 27, scope: !365)
!379 = !DILocation(line: 119, column: 20, scope: !365)
!380 = !DILocalVariable(name: "ch", scope: !365, file: !3, line: 120, type: !136)
!381 = !DILocation(line: 120, column: 17, scope: !365)
!382 = !DILocalVariable(name: "retval", scope: !365, file: !3, line: 121, type: !77)
!383 = !DILocation(line: 121, column: 17, scope: !365)
!384 = !DILocation(line: 122, column: 5, scope: !365)
!385 = !DILocation(line: 123, column: 20, scope: !386)
!386 = distinct !DILexicalBlock(scope: !387, file: !3, line: 122, column: 14)
!387 = distinct !DILexicalBlock(scope: !388, file: !3, line: 122, column: 5)
!388 = distinct !DILexicalBlock(scope: !365, file: !3, line: 122, column: 5)
!389 = !DILocation(line: 123, column: 14, scope: !386)
!390 = !DILocation(line: 123, column: 12, scope: !386)
!391 = !DILocation(line: 124, column: 13, scope: !392)
!392 = distinct !DILexicalBlock(scope: !386, file: !3, line: 124, column: 13)
!393 = !DILocation(line: 124, column: 16, scope: !392)
!394 = !DILocation(line: 124, column: 13, scope: !386)
!395 = !DILocation(line: 124, column: 24, scope: !392)
!396 = !DILocation(line: 125, column: 13, scope: !397)
!397 = distinct !DILexicalBlock(scope: !386, file: !3, line: 125, column: 13)
!398 = !DILocation(line: 125, column: 16, scope: !397)
!399 = !DILocation(line: 125, column: 13, scope: !386)
!400 = !DILocation(line: 125, column: 25, scope: !397)
!401 = distinct !{!401, !402, !403}
!402 = !DILocation(line: 122, column: 5, scope: !388)
!403 = !DILocation(line: 132, column: 5, scope: !388)
!404 = !DILocation(line: 126, column: 13, scope: !405)
!405 = distinct !DILexicalBlock(scope: !386, file: !3, line: 126, column: 13)
!406 = !DILocation(line: 126, column: 16, scope: !405)
!407 = !DILocation(line: 126, column: 13, scope: !386)
!408 = !DILocation(line: 126, column: 25, scope: !405)
!409 = !DILocation(line: 127, column: 13, scope: !410)
!410 = distinct !DILexicalBlock(scope: !386, file: !3, line: 127, column: 13)
!411 = !DILocation(line: 127, column: 19, scope: !410)
!412 = !DILocation(line: 127, column: 17, scope: !410)
!413 = !DILocation(line: 127, column: 13, scope: !386)
!414 = !DILocation(line: 128, column: 18, scope: !415)
!415 = distinct !DILexicalBlock(scope: !410, file: !3, line: 127, column: 25)
!416 = !DILocation(line: 129, column: 30, scope: !415)
!417 = !DILocation(line: 129, column: 38, scope: !415)
!418 = !DILocation(line: 129, column: 22, scope: !415)
!419 = !DILocation(line: 129, column: 20, scope: !415)
!420 = !DILocation(line: 130, column: 9, scope: !415)
!421 = !DILocation(line: 131, column: 25, scope: !386)
!422 = !DILocation(line: 131, column: 9, scope: !386)
!423 = !DILocation(line: 131, column: 19, scope: !386)
!424 = !DILocation(line: 131, column: 23, scope: !386)
!425 = !DILocation(line: 122, column: 5, scope: !387)
!426 = !DILocation(line: 133, column: 22, scope: !365)
!427 = !DILocation(line: 133, column: 30, scope: !365)
!428 = !DILocation(line: 133, column: 34, scope: !365)
!429 = !DILocation(line: 133, column: 14, scope: !365)
!430 = !DILocation(line: 133, column: 12, scope: !365)
!431 = !DILocation(line: 134, column: 5, scope: !365)
!432 = !DILocation(line: 134, column: 12, scope: !365)
!433 = !DILocation(line: 134, column: 17, scope: !365)
!434 = !DILocation(line: 135, column: 31, scope: !365)
!435 = !DILocation(line: 135, column: 14, scope: !365)
!436 = !DILocation(line: 135, column: 12, scope: !365)
!437 = !DILocation(line: 136, column: 10, scope: !365)
!438 = !DILocation(line: 136, column: 5, scope: !365)
!439 = !DILocation(line: 137, column: 12, scope: !365)
!440 = !DILocation(line: 137, column: 5, scope: !365)
!441 = distinct !DISubprogram(name: "repl", scope: !3, file: !3, line: 140, type: !442, scopeLine: 141, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!442 = !DISubroutineType(types: !443)
!443 = !{!8}
!444 = !DILocalVariable(name: "buffer", scope: !441, file: !3, line: 142, type: !445)
!445 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 131072, elements: !446)
!446 = !{!447}
!447 = !DISubrange(count: 16384)
!448 = !DILocation(line: 142, column: 10, scope: !441)
!449 = !DILocalVariable(name: "lil", scope: !441, file: !3, line: 143, type: !74)
!450 = !DILocation(line: 143, column: 11, scope: !441)
!451 = !DILocation(line: 143, column: 17, scope: !441)
!452 = !DILocation(line: 144, column: 18, scope: !441)
!453 = !DILocation(line: 144, column: 5, scope: !441)
!454 = !DILocation(line: 145, column: 18, scope: !441)
!455 = !DILocation(line: 145, column: 5, scope: !441)
!456 = !DILocation(line: 146, column: 18, scope: !441)
!457 = !DILocation(line: 146, column: 5, scope: !441)
!458 = !DILocation(line: 147, column: 5, scope: !441)
!459 = !DILocation(line: 148, column: 18, scope: !441)
!460 = !DILocation(line: 148, column: 5, scope: !441)
!461 = !DILocation(line: 149, column: 5, scope: !441)
!462 = !DILocation(line: 149, column: 12, scope: !441)
!463 = !DILocalVariable(name: "result", scope: !464, file: !3, line: 150, type: !77)
!464 = distinct !DILexicalBlock(scope: !441, file: !3, line: 149, column: 21)
!465 = !DILocation(line: 150, column: 21, scope: !464)
!466 = !DILocalVariable(name: "strres", scope: !464, file: !3, line: 151, type: !307)
!467 = !DILocation(line: 151, column: 21, scope: !464)
!468 = !DILocalVariable(name: "err_msg", scope: !464, file: !3, line: 152, type: !307)
!469 = !DILocation(line: 152, column: 21, scope: !464)
!470 = !DILocalVariable(name: "pos", scope: !464, file: !3, line: 153, type: !93)
!471 = !DILocation(line: 153, column: 16, scope: !464)
!472 = !DILocation(line: 154, column: 9, scope: !464)
!473 = !DILocation(line: 154, column: 19, scope: !464)
!474 = !DILocation(line: 155, column: 9, scope: !464)
!475 = !DILocation(line: 156, column: 20, scope: !476)
!476 = distinct !DILexicalBlock(scope: !464, file: !3, line: 156, column: 13)
!477 = !DILocation(line: 156, column: 35, scope: !476)
!478 = !DILocation(line: 156, column: 14, scope: !476)
!479 = !DILocation(line: 156, column: 13, scope: !464)
!480 = !DILocation(line: 156, column: 43, scope: !476)
!481 = !DILocation(line: 157, column: 28, scope: !464)
!482 = !DILocation(line: 157, column: 33, scope: !464)
!483 = !DILocation(line: 157, column: 18, scope: !464)
!484 = !DILocation(line: 157, column: 16, scope: !464)
!485 = !DILocation(line: 158, column: 32, scope: !464)
!486 = !DILocation(line: 158, column: 18, scope: !464)
!487 = !DILocation(line: 158, column: 16, scope: !464)
!488 = !DILocation(line: 159, column: 13, scope: !489)
!489 = distinct !DILexicalBlock(scope: !464, file: !3, line: 159, column: 13)
!490 = !DILocation(line: 159, column: 13, scope: !464)
!491 = !DILocation(line: 160, column: 28, scope: !489)
!492 = !DILocation(line: 160, column: 13, scope: !489)
!493 = !DILocation(line: 161, column: 24, scope: !464)
!494 = !DILocation(line: 161, column: 9, scope: !464)
!495 = !DILocation(line: 162, column: 23, scope: !496)
!496 = distinct !DILexicalBlock(scope: !464, file: !3, line: 162, column: 13)
!497 = !DILocation(line: 162, column: 13, scope: !496)
!498 = !DILocation(line: 162, column: 13, scope: !464)
!499 = !DILocation(line: 163, column: 46, scope: !500)
!500 = distinct !DILexicalBlock(scope: !496, file: !3, line: 162, column: 45)
!501 = !DILocation(line: 163, column: 41, scope: !500)
!502 = !DILocation(line: 163, column: 51, scope: !500)
!503 = !DILocation(line: 163, column: 13, scope: !500)
!504 = !DILocation(line: 164, column: 9, scope: !500)
!505 = distinct !{!505, !461, !506, !210}
!506 = !DILocation(line: 165, column: 5, scope: !441)
!507 = !DILocation(line: 166, column: 14, scope: !441)
!508 = !DILocation(line: 166, column: 5, scope: !441)
!509 = !DILocation(line: 167, column: 12, scope: !441)
!510 = !DILocation(line: 167, column: 5, scope: !441)
!511 = distinct !DISubprogram(name: "nonint", scope: !3, file: !3, line: 170, type: !512, scopeLine: 171, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !2, retainedNodes: !32)
!512 = !DISubroutineType(types: !513)
!513 = !{!8, !8, !306}
!514 = !DILocalVariable(name: "argc", arg: 1, scope: !511, file: !3, line: 170, type: !8)
!515 = !DILocation(line: 170, column: 23, scope: !511)
!516 = !DILocalVariable(name: "argv", arg: 2, scope: !511, file: !3, line: 170, type: !306)
!517 = !DILocation(line: 170, column: 41, scope: !511)
!518 = !DILocalVariable(name: "lil", scope: !511, file: !3, line: 172, type: !74)
!519 = !DILocation(line: 172, column: 11, scope: !511)
!520 = !DILocation(line: 172, column: 17, scope: !511)
!521 = !DILocalVariable(name: "filename", scope: !511, file: !3, line: 173, type: !307)
!522 = !DILocation(line: 173, column: 17, scope: !511)
!523 = !DILocation(line: 173, column: 28, scope: !511)
!524 = !DILocalVariable(name: "err_msg", scope: !511, file: !3, line: 174, type: !307)
!525 = !DILocation(line: 174, column: 17, scope: !511)
!526 = !DILocalVariable(name: "pos", scope: !511, file: !3, line: 175, type: !93)
!527 = !DILocation(line: 175, column: 12, scope: !511)
!528 = !DILocalVariable(name: "arglist", scope: !511, file: !3, line: 176, type: !529)
!529 = !DIDerivedType(tag: DW_TAG_typedef, name: "lil_list_t", file: !13, line: 78, baseType: !530)
!530 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !531, size: 64)
!531 = !DICompositeType(tag: DW_TAG_structure_type, name: "_lil_list_t", file: !13, line: 78, flags: DIFlagFwdDecl)
!532 = !DILocation(line: 176, column: 16, scope: !511)
!533 = !DILocation(line: 176, column: 26, scope: !511)
!534 = !DILocalVariable(name: "args", scope: !511, file: !3, line: 177, type: !77)
!535 = !DILocation(line: 177, column: 17, scope: !511)
!536 = !DILocalVariable(name: "result", scope: !511, file: !3, line: 177, type: !77)
!537 = !DILocation(line: 177, column: 23, scope: !511)
!538 = !DILocalVariable(name: "tmpcode", scope: !511, file: !3, line: 178, type: !11)
!539 = !DILocation(line: 178, column: 11, scope: !511)
!540 = !DILocalVariable(name: "i", scope: !511, file: !3, line: 179, type: !8)
!541 = !DILocation(line: 179, column: 9, scope: !511)
!542 = !DILocation(line: 180, column: 18, scope: !511)
!543 = !DILocation(line: 180, column: 5, scope: !511)
!544 = !DILocation(line: 181, column: 18, scope: !511)
!545 = !DILocation(line: 181, column: 5, scope: !511)
!546 = !DILocation(line: 182, column: 11, scope: !547)
!547 = distinct !DILexicalBlock(scope: !511, file: !3, line: 182, column: 5)
!548 = !DILocation(line: 182, column: 10, scope: !547)
!549 = !DILocation(line: 182, column: 15, scope: !550)
!550 = distinct !DILexicalBlock(scope: !547, file: !3, line: 182, column: 5)
!551 = !DILocation(line: 182, column: 17, scope: !550)
!552 = !DILocation(line: 182, column: 16, scope: !550)
!553 = !DILocation(line: 182, column: 5, scope: !547)
!554 = !DILocation(line: 183, column: 25, scope: !555)
!555 = distinct !DILexicalBlock(scope: !550, file: !3, line: 182, column: 28)
!556 = !DILocation(line: 183, column: 51, scope: !555)
!557 = !DILocation(line: 183, column: 56, scope: !555)
!558 = !DILocation(line: 183, column: 34, scope: !555)
!559 = !DILocation(line: 183, column: 9, scope: !555)
!560 = !DILocation(line: 184, column: 5, scope: !555)
!561 = !DILocation(line: 182, column: 24, scope: !550)
!562 = !DILocation(line: 182, column: 5, scope: !550)
!563 = distinct !{!563, !553, !564, !210}
!564 = !DILocation(line: 184, column: 5, scope: !547)
!565 = !DILocation(line: 185, column: 30, scope: !511)
!566 = !DILocation(line: 185, column: 12, scope: !511)
!567 = !DILocation(line: 185, column: 10, scope: !511)
!568 = !DILocation(line: 186, column: 19, scope: !511)
!569 = !DILocation(line: 186, column: 5, scope: !511)
!570 = !DILocation(line: 187, column: 17, scope: !511)
!571 = !DILocation(line: 187, column: 30, scope: !511)
!572 = !DILocation(line: 187, column: 5, scope: !511)
!573 = !DILocation(line: 188, column: 20, scope: !511)
!574 = !DILocation(line: 188, column: 5, scope: !511)
!575 = !DILocation(line: 189, column: 29, scope: !511)
!576 = !DILocation(line: 189, column: 22, scope: !511)
!577 = !DILocation(line: 189, column: 39, scope: !511)
!578 = !DILocation(line: 189, column: 15, scope: !511)
!579 = !DILocation(line: 189, column: 13, scope: !511)
!580 = !DILocation(line: 190, column: 13, scope: !511)
!581 = !DILocation(line: 190, column: 182, scope: !511)
!582 = !DILocation(line: 190, column: 5, scope: !511)
!583 = !DILocation(line: 191, column: 24, scope: !511)
!584 = !DILocation(line: 191, column: 29, scope: !511)
!585 = !DILocation(line: 191, column: 14, scope: !511)
!586 = !DILocation(line: 191, column: 12, scope: !511)
!587 = !DILocation(line: 192, column: 10, scope: !511)
!588 = !DILocation(line: 192, column: 5, scope: !511)
!589 = !DILocation(line: 193, column: 20, scope: !511)
!590 = !DILocation(line: 193, column: 5, scope: !511)
!591 = !DILocation(line: 194, column: 19, scope: !592)
!592 = distinct !DILexicalBlock(scope: !511, file: !3, line: 194, column: 9)
!593 = !DILocation(line: 194, column: 9, scope: !592)
!594 = !DILocation(line: 194, column: 9, scope: !511)
!595 = !DILocation(line: 195, column: 17, scope: !596)
!596 = distinct !DILexicalBlock(scope: !592, file: !3, line: 194, column: 41)
!597 = !DILocation(line: 195, column: 56, scope: !596)
!598 = !DILocation(line: 195, column: 51, scope: !596)
!599 = !DILocation(line: 195, column: 61, scope: !596)
!600 = !DILocation(line: 195, column: 9, scope: !596)
!601 = !DILocation(line: 196, column: 5, scope: !596)
!602 = !DILocation(line: 197, column: 14, scope: !511)
!603 = !DILocation(line: 197, column: 5, scope: !511)
!604 = !DILocation(line: 198, column: 12, scope: !511)
!605 = !DILocation(line: 198, column: 5, scope: !511)
!606 = distinct !DISubprogram(name: "main", scope: !3, file: !3, line: 201, type: !512, scopeLine: 202, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !32)
!607 = !DILocalVariable(name: "argc", arg: 1, scope: !606, file: !3, line: 201, type: !8)
!608 = !DILocation(line: 201, column: 14, scope: !606)
!609 = !DILocalVariable(name: "argv", arg: 2, scope: !606, file: !3, line: 201, type: !306)
!610 = !DILocation(line: 201, column: 32, scope: !606)
!611 = !DILocation(line: 203, column: 9, scope: !612)
!612 = distinct !DILexicalBlock(scope: !606, file: !3, line: 203, column: 9)
!613 = !DILocation(line: 203, column: 14, scope: !612)
!614 = !DILocation(line: 203, column: 9, scope: !606)
!615 = !DILocation(line: 203, column: 26, scope: !612)
!616 = !DILocation(line: 203, column: 19, scope: !612)
!617 = !DILocation(line: 204, column: 24, scope: !612)
!618 = !DILocation(line: 204, column: 30, scope: !612)
!619 = !DILocation(line: 204, column: 17, scope: !612)
!620 = !DILocation(line: 204, column: 10, scope: !612)
!621 = !DILocation(line: 205, column: 1, scope: !606)

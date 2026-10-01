指针分析的敏感性：
上下文敏感（需要关注函数之间的调用关系），类型敏感 (区分 struct A* 和 struct B*, 或者struct File*)，和 域敏感（区分不同的结构体成员）。
流不敏感（不区分代码执行的顺序），路径不敏感（不区分代码执行的不同分支），
Pointer-Aware Knowledge Graph
![KnowledgeGraph](./image.png)

## 要做哪些分析 -- 为什么要做这些分析
分析对象 -- 分析内容 -- 为什么要做这种分析
静态分析得到分析结果，大模型根据静态分析结果得到推理的结果。

##### 1 所有权分析 <pointer, is/is_not, owing pointer> 
> 拥有型指针（owning pointer）拥有其所指向的数据，当拥有型指针超出作用域时，其指向的数据会被自动清理（即调用 drop 函数）。而非拥有型指针（non-owning pointer），也称为借用指针（borrowed pointer），并不拥有数据，它只是临时引用数据，不能决定数据的生命周期，数据的生命周期由拥有它的指针来决定。

- Object: 函数参数中的指针，结构体成员当中的指针，union成员当中的指针
- What: 
Q: 拥有型指针: 什么时候使用使用 Box, 何时使用 T:
分配在堆上 ——> Box; 反之--> T
Q: 什么时候使用 &T/&mut T，什么时候使用 &Box<T>/&mut Box<T>
一般情况下： &T / &mut T (首选的借用方式)
当你看到 C 代码使用了指向指针的指针 (T**)，并且其目的是修改调用者原始指针变量所指向的堆分配内存: &Box<T> / &mut Box<T>

- How: 
结构体｜union 成员指针：查看下面例子
函数返回指针: 接受返回值的函数，在函数内部是否对接受的返回值进行进行free(). 
*如果当前函数对指针进行了free操作，则认为是拥有所有权，反之则不认为*
函数指针所有权判定：*若指针在函数调用后不再使用，且调用方函数内释放了指针的内存，则所有权确实移交，对应 Rust 的 Box<T>。*
另外：&mut Box<T> 是对 Box 的可变借用，用于修改 Box 本身或内容，而非转移所有权。
```c
// C代码：明确移交所有权
void take_ownership(int* ptr) { free(ptr); }
void caller() {
    int* p = malloc(sizeof(int));
    take_ownership(p); // p不再使用，所有权移交
}

// Rust翻译
fn take_ownership(ptr: Box<i32>) { /* 自动释放 */ }
fn caller() {
    let p = Box::new(42);
    take_ownership(p); // p所有权转移
}
```
结构体/联合体成员指针的所有权判定：
如果结构体成员中的数据是通过·指向/借用·外部数据进行初始化，则不拥有数据的所有权（对应 &T/&mut T），反之则拥有数据的所有权（通过malloc初始化的）。
> 使用结构体类型，来进行释放。例如： DataWrapper* w --> free(w->data)
> 反之则不拥有所有权
示例 1：结构体拥有所有权
```c
// C代码：结构体管理内存
struct DataWrapper {
    int* data;
};
void init_wrapper(struct DataWrapper* w) {
    w->data = malloc(sizeof(int)); // 分配内存
}
void destroy_wrapper(struct DataWrapper* w) {
    free(w->data); // 释放内存
}
```
```rust
// Rust翻译：使用Box
struct DataWrapper {
    data: Box<i32>,
}
impl DataWrapper {
    fn new() -> Self {
        DataWrapper { data: Box::new(0) }
    }
}
// Drop trait自动释放，无需手动析构
```
示例 2：结构体借用外部数据
```c
// C代码：结构体不管理内存
struct DataView {
    const int* data;
};
void init_view(struct DataView* v, const int* src) {
    v->data = src; // 借用外部数据
}
```
```rust
// Rust翻译：使用引用+生命周期标注
struct DataView<'a> {
    data: &'a i32,
}
impl<'a> DataView<'a> {
    fn new(data: &'a i32) -> Self {
        DataView { data }
    }
}
```
- Implementation: context and field sensitive; BottomUp analyzation. 分配数据和释放数据的方式；谁释放数据；如果没有free数据，就使用借用方式


##### 2 指向性分析 <pointer, point to, referent>
- Object: 函数参数的指针； 函数返回值的指针；结构体/union 成员当中的指针
- What: 通过大模型得到的指向性分析输出：
a. 两个指针是否可能为别名 --> 如果互为别名，且都是可变的，将这两个参数合并为一个参数，避免使用RefCell
b. 函数的返回值是不是指向 函数参数的返回值 --> 要不要加生命周期的标注 [注意，参数都是引用的前提下]
c. 如果结构体成员当中的指针，指向了其他类型的值 [没有获得所有权] --> 加上生命周期的标注
- How: C 语言当中指针可以指向任意类型的数据（如function | struct | union | array | struct | primitive type | pointer），需要分析指针指向的具体内容。
```c
// quadtree_node_free 中的第二个参数指向结果集为{%14, @elision_}
...
%13 = getelementptr inbounds %struct.quadtree, %struct.quadtree* %12, i32 0, i32 1, !dbg !1031
%14 = load void (i8*)*, void (i8*)** %13, align 8, !dbg !1031
call void @quadtree_node_free(%struct.quadtree_node* %11, void (i8*)* %14), !dbg !1032
...                                         ; preds = %1
...
call void @quadtree_node_free(%struct.quadtree_node* %18, void (i8*)* @elision_), !dbg !1037
```
- Implementation: context and field sensitive; BottomUp analyzation. 间接调用也要得到。例如： F3 call F2; F2 call F1; F1 call F0; F4 call F0。 在分析F0的时候。如果F0中的形参和F1中的形参一致，则处理F1. 同理，如果F1 中的形参和实参一致的话，则要处理F2. 依次向上处理。



##### 3 可变性分析: infers which pointers are used to modify the object they point to
- Object: 函数参数当中的指针所指向的内容是否被修改. 结构体｜union 当中的成员指针
- Why: 如果函数参数当中的指针所指向的内容需要被修改，则被修改则需要使用&mut 来
- Approah: 
函数参数指针：判定某个指针所对应的值，在函数内是否被修改。但是这里要注意，更深层次的修改。例如，F1 call F2, and F2 call F3. F1 的参数在F1中没有修改，并且传入到了F2, F2 中也没有修改，传入到了F3；但是F3中修改了。
函数返回指针：接受返回值的函数，在函数内部是否对接受的返回值进行了修改。
结构体｜union 成员指针：⚠️C当中可能涉及到

> 如果一个函数参数的存在两个指针，（1）这两个指针都指向同一个结构体（2）这两个指针所指向的值都是可变的。此时对于结构体成员而言，需要Rc<RefCell<T>>来声明

##### 4. 可空性分析：分析一个指针是否可能为会空
- Object: 函数参数当中的指针所指向的内容是否被修改. 结构体｜union 当中的成员指针, 函数返回值
- Why: 在Rust当中，如果一个指针可能指为空，则需要使用 Option 来声明
- Approach: 
函数参数指针：函数指针在C函数内部当中，是否进行了判定为空的操作
函数返回指针：接受返回值的函数，在C函数内部是否对接受的返回值进行了判定是否为空的操作
结构体｜union 成员指针：要全局判定，是否在C当中有判定是否为空的代码






##### 2. 指向性分析 <pointer, point to, referent>
- Object: 函数参数当中的指针; 结构体成员当中的指针？？；Union成员当中的指针？？
- Why: C 语言当中指针可以指向任意类型的数据（如int, struct, array, function等），需要分析指针指向的具体内容。
- Approach: 上下文敏感，路径不敏感。
```c
// quadtree_node_free 中的第二个参数指向结果集为{%14, @elision_}
...
%13 = getelementptr inbounds %struct.quadtree, %struct.quadtree* %12, i32 0, i32 1, !dbg !1031
%14 = load void (i8*)*, void (i8*)** %13, align 8, !dbg !1031
call void @quadtree_node_free(%struct.quadtree_node* %11, void (i8*)* %14), !dbg !1032
...                                         ; preds = %1
...
call void @quadtree_node_free(%struct.quadtree_node* %18, void (i8*)* @elision_), !dbg !1037
```





##### 3 所有权分析 <pointer, is/is_not, owing pointer> 
> 拥有型指针（owning pointer）拥有其所指向的数据，当拥有型指针超出作用域时，其指向的数据会被自动清理（即调用 drop 函数）。而非拥有型指针（non-owning pointer），也称为借用指针（borrowed pointer），并不拥有数据，它只是临时引用数据，不能决定数据的生命周期，数据的生命周期由拥有它的指针来决定。

- Object: 函数参数中的指针，结构体成员当中的指针，union成员当中的指针
- Why: 如果转交了所有权则需要使用Box<T> 来声明，如果未拥有所有权，则需要使用&T 或者&mut T
- Approach: 
结构体｜union 成员指针：查看下面例子
函数返回指针: 接受返回值的函数，在函数内部是否对接受的返回值进行进行free(). 
*如果当前函数对指针进行了free操作，则认为是拥有所有权，反之则不认为*
函数指针所有权判定：*若指针在函数调用后不再使用，且调用方函数内释放了指针的内存，则所有权确实移交，对应 Rust 的 Box<T>。*
另外：&mut Box<T> 是对 Box 的可变借用，用于修改 Box 本身或内容，而非转移所有权。
```c
// C代码：明确移交所有权
void take_ownership(int* ptr) { free(ptr); }
void caller() {
    int* p = malloc(sizeof(int));
    take_ownership(p); // p不再使用，所有权移交
}

// Rust翻译
fn take_ownership(ptr: Box<i32>) { /* 自动释放 */ }
fn caller() {
    let p = Box::new(42);
    take_ownership(p); // p所有权转移
}
```
结构体/联合体成员指针的所有权判定：
如果结构体成员中的数据是通过·指向/借用·外部数据进行初始化，则不拥有数据的所有权（对应 &T/&mut T），反之则拥有数据的所有权（通过malloc初始化的）。
> 使用结构体类型，来进行释放。例如： DataWrapper* w --> free(w->data)
> 反之则不拥有所有权
示例 1：结构体拥有所有权
```c
// C代码：结构体管理内存
struct DataWrapper {
    int* data;
};
void init_wrapper(struct DataWrapper* w) {
    w->data = malloc(sizeof(int)); // 分配内存
}
void destroy_wrapper(struct DataWrapper* w) {
    free(w->data); // 释放内存
}
```
```rust
// Rust翻译：使用Box
struct DataWrapper {
    data: Box<i32>,
}
impl DataWrapper {
    fn new() -> Self {
        DataWrapper { data: Box::new(0) }
    }
}
// Drop trait自动释放，无需手动析构
```
示例 2：结构体借用外部数据
```c
// C代码：结构体不管理内存
struct DataView {
    const int* data;
};
void init_view(struct DataView* v, const int* src) {
    v->data = src; // 借用外部数据
}
```
```rust
// Rust翻译：使用引用+生命周期标注
struct DataView<'a> {
    data: &'a i32,
}
impl<'a> DataView<'a> {
    fn new(data: &'a i32) -> Self {
        DataView { data }
    }
}
```

##### 4 可变性分析: infers which pointers are used to modify the object they point to
- Object: 函数参数当中的指针所指向的内容是否被修改. 结构体｜union 当中的成员指针
- Why: 如果函数参数当中的指针所指向的内容需要被修改，则被修改则需要使用&mut 来
- Approah: 
函数参数指针：判定某个指针所对应的值，在函数内是否被修改。但是这里要注意，更深层次的修改。例如，F1 call F2, and F2 call F3. F1 的参数在F1中没有修改，并且传入到了F2, F2 中也没有修改，传入到了F3；但是F3中修改了。
函数返回指针：接受返回值的函数，在函数内部是否对接受的返回值进行了修改。
结构体｜union 成员指针：⚠️C当中可能涉及到

> 如果一个函数参数的存在两个指针，（1）这两个指针都指向同一个结构体（2）这两个指针所指向的值都是可变的。此时对于结构体成员而言，需要Rc<RefCell<T>>来声明

##### 5. 可空性分析：分析一个指针是否可能为会空
- Object: 函数参数当中的指针所指向的内容是否被修改. 结构体｜union 当中的成员指针, 函数返回值
- Why: 在Rust当中，如果一个指针可能指为空，则需要使用 Option 来声明
- Approach: 
函数参数指针：函数指针在C函数内部当中，是否进行了判定为空的操作
函数返回指针：接受返回值的函数，在C函数内部是否对接受的返回值进行了判定是否为空的操作
结构体｜union 成员指针：要全局判定，是否在C当中有判定是否为空的代码

##### 1. 别名分析 <pointer, may/must/no alias, pointer>
- Object：函数当中的参数当中包含的多个指针（if any）
- Why: Rust 函数参数当中不允许别名的存在. 需要分析函数参数当中的指针是否存在别名关系.
- Approach：上下文相关的分析. 和调用点相关
```c
...
store i32* %x, i32** %p1, align 8, !dbg !709
store i32* %x, i32** %p2, align 8, !dbg !711
%0 = load i32*, i32** %p1, align 8, !dbg !712 // %0 从P1所指向的地址加载过来的
%1 = load i32*, i32** %p2, align 8, !dbg !713 // %1 从P2所指向的地址加载过来的
// 所以 %0 和 %1 是别名关系；也就是F2的前两个参数是别名关系
call void @F2(i32* noundef %0, i32* noundef %1, i32 noundef 5, i32 noundef 10), !dbg !714
...
store i32* null, i32** %p3, align 8, !dbg !716
%2 = load i32*, i32** %p3, align 8, !dbg !717 // %2 从P3所指向的地址加载过来的
%3 = load i32*, i32** %p2, align 8, !dbg !718 // %3 从P2所指向的地址加载过来
// 所以 %2 和 %3 不是别名关系；也就是F2的前两个参数不是别名关系
call void @F2(i32* noundef %2, i32* noundef %3, i32 noundef 15, i32 noundef 20), !dbg !719
```



#### 结构体分析
1. 指向性分析 -- ❌
2. 





owning pointer / borrowed pointer
#### 将根据如下思路实现代码，来判定C function当中的函数参数指针是Own(p) 还是 Borr(p)
对于C project 当中的函数，先获取到函数调用图。然后自底向上（callee to caller）进行分析，并且在分析的时候将底层信息传递给上层分析结果。
1. 如何判定一个函数中的参数为一个拥有型指针：
函数参数指针 (T* p)：目标是判断函数 f(T* p) 是否“消耗”或“取得” p 的所有权。

(1) 显式释放： 函数 f 内部直接或间接调用 free(p) (或对应的自定义释放函数，如 my_type_free(p)）。
- 如果存在 --> Own(p) (函数消耗了指针)。
- 反之转 (2) 。
```c
// 也有可能通过函数指针进行释放 
void cleanup(void* p) { free(p); }
void f(int* p, void (*cb)(void*)) {
    cb(p);  // 可能释放p，但需分析所有可能的cb实现
}
```
(2) 持久化存储并承担管理责任：函数 f 将指针 p 存储到：
- 如果存在以下情况，并且伴随着管理责任的转移 --> Own(p) (函数将所有权转移给了全局上下文、结构体实例或容器)。
    a.全局变量中，并且有明确的机制（如程序退出时、或特定API调用时）会释放这个全局变量指向的内存。
    b. 传递给它的某个结构体实例的字段中 (some_struct->field = p;)，并且该结构体的“销毁”或“清理”函数会负责释放 some_struct->field 指向的内存。
    c. 传递给它的某个集合/容器中（如添加到链表、哈希表），并且该容器的销毁或元素移除逻辑会负责释放 p 指向的内存。
- 反之转 (3)。

(3) 如果以上情况都不满足，则该指针参数更可能是借用型。
- Borr(p) (函数仅仅是使用（读取/修改）指针指向的数据，但不负责其生命周期)。

注意：关于函数参数当中的结构体指针参数分析，应该是field sensitive的
```c
typedef struct quadtree_node_t {
    quadtree_point_t *point;
    void *key;
} quadtree_node_t;

void quadtree_node_reset(quadtree_node_t *node, void (*key_free)(void *)) {
    quadtree_point_free(node->point);  // Own(node->point)
    (*key_free)(node->key);            // Own(node->key)
    // Borr(node)
}

void quadtree_point_free(quadtree_point_t *point) {
    free(point); // Own(point) 因为显示释放了point
}
```

2. 关于链式调用：
传递给拥有型参数： 如果函数 f(T* p1) 调用了函数 g(T* p2)，并且 p1 被用作 p2（即 g(p1)），若参数 p2 对于函数 g 来说是拥有型的（即 g 会消耗 p2），那么参数 p1 对于函数 f 来说也必须被视为拥有型。函数 f 实际上是将 p1 的所有权传递给了 g。
传递给借用型参数： 如果参数 p2 对于函数 g 来说是借用型的，那么 p1 对于函数 f 是拥有型还是借用型，则需要根据上述针对 f 的参数指针的规则（1-3）独立判定。g 的借用行为不改变 f 对 p1 的所有权承诺



4. 默认保守：
在不确定参数是否为拥有型时，在向Rust转换的初期，更安全的做法是先假设它是借用型（Rust中的 &T 或 &mut T）。如果后续分析或测试表明C代码确实转移了所有权或释放了内存，再将其调整为Rust中的拥有型参数（如 Box<T>, Vec<T>, String）。



#### 结构体中的成员指针是否为拥有型指针
规则1: 这个指针的赋值通过 malloc，calloc、realloc、strdup，strndup 函数进行赋值。
规则2: 这个指针会通过 free 进行释放。
满足上述两条中的任一, 则这个成员指针为Own, 反之则为Brow



#### 对于任何一个返回指针的C函数 f, 其返回值是否为拥有型指针。
ReturnP 是否来自 malloc 或类似的分配函数？
是 -> 拥有型 (Owning)。
否 -> 继续。
ReturnP 是否来自函数输入参数、全局/静态变量、或字符串字面量？
是 -> 借用型 (Borrowed)。
否 -> 继续。
p 是否来自另一个函数 g 的调用？
是 -> 对 g 重复此分析流程。f 的返回值类型与 g 相同。
否 -> 继续。

⚠️：如果一个函数，存在多个返回语句，只要有一个return 返回的对象是拥有型指针，则默认这个函数返回的就是一个拥有型指针。


#### 关于生命周期的标注。
##### 关于Rust 函数是否要进行生命周期的标注
1. 当一个函数返回一个引用（在C中为指针），并且该引用的来源对于编译器来说是模糊不清的（即它可能来自多个输入引用中的任何一个），那么就需要手动标注生命周期来消除这种歧义。




#### 结构体成员当中的指针是否为可变的
判断流程：一个三步侦查法
1. 第一步：检查 const 关键字（最直接的线索） [默认不可变，这步省略]
这是 C 语言为我们提供的最明显的线索。

如果指针被声明为 const T* member：

含义：这是一个指向“常量数据”的指针。你不应该通过这个指针来修改它所指向的数据。
结论：这几乎总是应该被翻译成一个不可变借用 &'a T。
```c
typedef struct {
    const char* name; // 指向一个不可变的 char 序列
} UserView;
// Rust -> struct UserView<'a> { name: &'a str }
```
如果指针被声明为 T* member (没有 const)：

含义：这是最模糊的情况。它可能被用来修改数据，也可能只是用于读取。const 的缺失不代表它一定是可变的。
结论：进入第二步和第三步，进行更深入的行为分析。
注意：T *const member 的意思是 member 这个指针本身不能被修改指向别处，但它指向的数据是可以被修改的。所以 T *const 仍然需要我们进行下一步分析。

2. 第二步：分析函数签名（契约线索）[这步也可以省略，因为默认不可变]
检查所有使用了这个结构体实例或其成员指针的函数，它们的“契约”（函数签名）是如何定义的。

传递结构体实例：

如果一个函数接收整个结构体实例的指针是 const MyStruct* s，那么它很可能不会修改 s 的任何内容，包括通过 s->member 指针修改数据。这是不可变的信号。
直接传递成员指针：

观察接收这个成员指针的函数参数。
```c
// C 代码
void print_data(const data_t* d); // 明确表示只读
void update_data(data_t* d);      // 没有 const，暗示可能修改
```
如果你的 s->member 总是被传递给像 print_data 这样的函数，那么它很可能是不可变的 &T。
如果它哪怕只有一次被传递给了像 update_data 这样（被证实会修改数据的）函数，那么它就必须是可变的 &mut T。

3. 第三步：分析代码行为（“作案现场”的证据）
这是最关键的一步，你需要找到代码中实际发生修改的“作案现场”。以下是可变性 (&mut T) 的确凿证据（“Red Flags”）：

赋值操作：指针指向的数据出现在赋值符号 (=) 的左边。

```c
s->member->field = 42;
strcpy(s->member->name, "new_name");
```
修改型函数：指针被传递给了明确会修改内容的标准库函数或自定义函数。

标准库：strcpy, strcat, memset, memcpy (目标参数), scanf 等。
自定义：node_set_value(s->member, 100);
自增/自减等修改性操作：
```c
(*s->member)++;
s->member->counter += 1;
获取地址用于修改：为了修改指针，而获取了它的地址。

// 比如一个函数需要修改指针本身指向的位置
reassign_node_pointer(&s->member);
```
如果没有找到以上任何一条“作案”证据，并且所有对该成员的使用都是读取字段、比较、打印等只读行为，那么你就可以安全地将其归类为不可变借用 &'a T。


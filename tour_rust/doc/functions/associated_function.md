## 关联函数（Associated Functions）解析

### 一、概念本质

- 关联函数是**与类型关联的函数**，而非类型的实例

- 语法：`Type::function_name()`

- 类比其他语言：

  | 语言    | 类似概念               | 示例                |
  | ------- | ---------------------- | ------------------- |
  | Rust    | 关联函数               | `String::new()`     |
  | Java/C# | 静态方法               | `String.valueOf()`  |
  | C++     | 静态成员函数           | `std::string::npos` |
  | Python  | 类方法（@classmethod） | `str.join()`        |

### 二、 `new` 函数详解

```rust
let s = String::new(); // 创建空字符串
```

- **功能**：创建新的空 `String` 实例

- **内存行为**：

  - 初始容量为 0
  - 首次添加内容时分配内存

- **等效代码**：

  ```rust
  // 手动实现类似功能
  let s = String::with_capacity(0);
  ```

### 三、为什么使用 `::` 语法？

- 明确区分类型函数与实例方法

- 对比实例方法调用：

  ```rust
  // 关联函数（类型调用）
  let s = String::new();
  
  // 实例方法（实例调用）
  s.push_str("text");
  ```
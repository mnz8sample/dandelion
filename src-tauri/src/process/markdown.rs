use comrak::{nodes::AstNode, parse_document, Arena, ComrakOptions};
use std::fs::File;
use std::io::{self, Read};

// 函数接收 Arena 作为参数并返回 AstNode
pub fn parse_markdown_file<'a>(file_path: &str, arena: &'a Arena<AstNode<'a>>) -> io::Result<&'a AstNode<'a>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let options = ComrakOptions::default();
    let ast = parse_document(arena, &contents, &options);

    Ok(ast)
}

/// 读取文件并将其解析为 AST
pub fn parse_markdown_file2<'a>(file_path: &str, arena: &'a Arena<AstNode<'a>>) -> io::Result<&'a AstNode<'a>> {
    // 尝试打开文件，如果失败则返回错误
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    // 读取文件内容
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let options = ComrakOptions::default();
    let ast = parse_document(arena, &contents, &options);

    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let arena = Arena::new();
        let ast_result = parse_markdown_file2("a.md", &arena);
        // Properly handle the Result before accessing the AstNode
        match ast_result {
            // Ok(ast) => println!("Root node type: {:?}", ast.data.borrow().value),
            Ok(ast) => println!("Root node type: {:?}", ast),
            Err(e) => println!("Error reading markdown file: {}", e),
        }
    }
}

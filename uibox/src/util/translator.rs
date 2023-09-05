use std::collections::HashMap;
use crate::config;

pub fn tr(text: &str) -> String {
    if config::ui().language == "cn" {
        return text.to_string();
    }

    let mut items: HashMap<&str, &str> = HashMap::new();
    items.insert("出错", "Error");
    items.insert("原因", "Reason");
    items.insert("新建成功", "New Session Success");
    items.insert("删除成功", "Delete Success");
    items.insert("删除失败", "Delete Failed");
    items.insert("复制失败", "Copy Failed");
    items.insert("复制成功", "Copy Success");
    items.insert("保存到数据库失败", "Save to Database Failed");

    items.insert("重置成功", "Reset Success");
    items.insert("保存失败", "Save Failed");
    items.insert("保存成功", "Save Success");
    items.insert("隐藏程序失败", "Hide Window Failed");
    items.insert("清除缓存失败", "Clean Cache Failed");
    items.insert("清除缓存成功", "Clean Cache Success");
    items.insert("正在重试...", "retrying...");
    items.insert("图片格式非法！", "Invalid Image Format!");
    items.insert("打开文件失败！", "Open File Failed!");

    if let Some(txt) = items.get(text) {
        return txt.to_string();
    }

    text.to_string()
}

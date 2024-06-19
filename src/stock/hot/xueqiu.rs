

// """
// 雪球-沪深股市-热度排行榜-关注排行榜
// https://xueqiu.com/hq
// :param symbol: choice of {"本周新增", "最热门"}
// :type symbol: str
// :return: 关注排行榜
// :rtype: pandas.DataFrame
// """
pub fn getHotFollowXueqiu() {
    
    let url = "https://xueqiu.com/service/v5/stock/screener/screen";
    let mut response = ureq::get(url)
    .query("category", "CN")
    .query("size", "200")
    .query("order", "desc")
    .query("order_by", "follow")
    .query("only_count", "0")
    .call();
}
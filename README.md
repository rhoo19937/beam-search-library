高速なビームサーチライブラリです  

説明  
nop_hash: 何もしないハッシュ関数のHashMap,HashSet  
partial_sort: ヒープソートによるsort,select_nth,partial_sort  
select_nth: Rustのバージョンが低くても最悪O(N)の選択アルゴリズムを使えるようにするやつ  


normal_beam: 普通のビームサーチ  
incremental_beam: 1手だけ差分更新するビームサーチ  
tree_beam: 毎回状態をシュミレートするビームサーチ  


*_multi: dp[i]からdp[i+1]以外へ遷移することも可能なビームサーチ  


すべてのビームサーチについて共通で以下の制約を満たす必要があります。  
- 一つのNodeから遷移先は255個以下
- ビーム幅が常にMAX_WIDTHを超えない
- 同時に存在するノードが常にMAX_NODESを超えない
- MAX_NODES <= uint::MAX


最初以外の制約は多分assertか境界チェックに引っかかるはずです。  


頑張って各々何問かverifyしましたが、普通にバグってるかもしれません。  
import { formatDistanceToNow } from 'date-fns';
import { enUS, zhCN, ja } from 'date-fns/locale';
console.log(formatDistanceToNow(new Date(Date.now() - 60000), { addSuffix: true, locale: zhCN }));

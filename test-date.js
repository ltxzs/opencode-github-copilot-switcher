import { formatDistanceToNow } from 'date-fns';
import { enUS, zhCN, ja } from 'date-fns/locale';
console.log(formatDistanceToNow(new Date(), { addSuffix: true, locale: zhCN }));

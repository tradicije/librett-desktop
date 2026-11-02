// Keep this identity rule aligned with domain::club_key. Typos need an explicit choice.
export function clubKey(name: string): string {
  const words=name.toLowerCase().replace(/[šś]/g,'s').replace(/[čć]/g,'c').replace(/ž/g,'z').replace(/đ/g,'d').replace(/[^\p{L}\p{N}]+/gu,' ').trim().split(/\s+/);
  if(words.slice(0,3).join(' ')==='stoni teniski klub')words.splice(0,3);
  else if(words[0]==='stk')words.shift();
  return words.join('');
}
function oneEditApart(a: string,b: string): boolean {
  const first=[...a],second=[...b];
  if(Math.abs(first.length-second.length)>1)return false;
  let i=0,j=0,edits=0;
  while(i<first.length && j<second.length){
    if(first[i]===second[j]){i++;j++;continue;}
    if(++edits>1)return false;
    if(first.length>=second.length)i++;
    if(second.length>=first.length)j++;
  }
  return edits+(first.length-i)+(second.length-j)===1;
}
export function clubSuggestion(input: string,known: string[]): string|null {
  const key=clubKey(input);
  if(!key)return null;
  const exact=known.find(name=>clubKey(name)===key);
  if(exact)return exact===input.trim()?null:exact;
  if([...key].length<5)return null;
  const close=known.filter(name=>oneEditApart(key,clubKey(name)));
  return close.length===1?close[0]:null;
}

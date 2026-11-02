import { describe, expect, it } from 'vitest';
import { escapeHtml, csvCell, csv } from './report-format';

describe('escapeHtml', () => {
  it('escapes markup and quotes while preserving Serbian text', () => {
    expect(escapeHtml('<script>"Đorđe" & Željko\'</script>'))
      .toBe('&lt;script&gt;&quot;Đorđe&quot; &amp; Željko&#39;&lt;/script&gt;');
  });
});

describe('csvCell', () => {
  it.each([
    ['=SUM(A1:A2)', '"\'=SUM(A1:A2)"'],
    [' +cmd', '"\' +cmd"'],
    ['@SUM(1)', '"\'@SUM(1)"'],
  ])('neutralizes a spreadsheet formula in %j', (value, expected) => {
    expect(csvCell(value)).toBe(expected);
  });

  it('keeps negative numeric values numeric', () => {
    expect(csvCell(-12)).toBe('"-12"');
  });

  it('quotes commas and embedded quotation marks', () => {
    expect(csvCell('Bubušinac, "Đorđe"')).toBe('"Bubušinac, ""Đorđe"""');
  });
});

describe('csv', () => {
  it('writes a BOM, CRLF rows and intact Serbian text with embedded newlines', () => {
    expect(csv([['Igrač', 'Klub'], ['Đorđe\nŽeljko', 'BUB']]))
      .toBe('\uFEFF"Igrač","Klub"\r\n"Đorđe\nŽeljko","BUB"\r\n');
  });
});

#include <stdbool.h>
#include <stdio.h>

#include "util.h"

#include "qfplib-m0-full.h"

static bool isnumeric(const char c);

static bool isnumeric(const char c) {
  if (('0' <= c) && ('9' >= c)) {
    return true;
  }
  return false;
}

uint32_t utilAbs(const int32_t x) { return (x < 0) ? -x : x; }

void utilStrReverse(char *pBuf, size_t len) {
  char     tmp;
  uint32_t idxEnd = len - 1u;
  for (uint32_t idx = 0; idx < (len / 2); idx++) {
    tmp          = pBuf[idx];
    pBuf[idx]    = pBuf[idxEnd];
    pBuf[idxEnd] = tmp;
    idxEnd--;
  }
}

size_t utilStrlen(const char *pBuf) {
  size_t charCnt = 0;
  while (*pBuf++) {
    charCnt++;
  }
  return charCnt;
}

size_t utilUtoa(char *pBuf, uint32_t val, const ITOA_BASE_t base) {
  size_t      charCnt = 0;
  char *const pBase   = pBuf;

  /* Handle 0 explicitly */
  if (0 == val) {
    *pBuf++ = '0';
    *pBuf   = '\0';
    return 2u;
  }

  if (ITOA_BASE10 == base) {
    while (0 != val) {
      *pBuf++ = (char)((val % 10u) + '0');
      val     = val / 10u;
      charCnt++;
    }
  } else {
    const char itohex[] = "0123456789abcdef";
    while (0 != val) {
      *pBuf++ = itohex[(val & 0xFu)];
      val >>= 4;
      charCnt++;
    }
  }

  /* Terminate and return */
  *pBuf = '\0';
  charCnt++;

  utilStrReverse(pBase, charCnt - 1u);
  return charCnt;
}

size_t utilItoa(char *pBuf, int32_t val, const ITOA_BASE_t base) {
  /* Hex treats value as unsigned bit pattern */
  if (ITOA_BASE16 == base) {
    return utilUtoa(pBuf, (uint32_t)val, base);
  }

  /* Negative decimal: write sign, then magnitude */
  if (val < 0) {
    *pBuf = '-';
    return utilUtoa(pBuf + 1, (uint32_t)(-val), base) + 1u;
  }

  return utilUtoa(pBuf, (uint32_t)val, base);
}

ConvInt_t utilAtoi(char *pBuf, const ITOA_BASE_t base) {
  bool      isNegative = false;
  size_t    len;
  uint32_t  mulCnt = 1;
  ConvInt_t conv   = {false, 0};

  if ('-' == *pBuf) {
    isNegative = true;
    pBuf++;
  }

  /* Reverse string and convert */
  len = utilStrlen(pBuf);
  utilStrReverse(pBuf, len);

  if (ITOA_BASE10 == base) {
    while (*pBuf) {
      if (!isnumeric(*pBuf)) {
        return conv;
      }
      conv.val += ((*pBuf++) - '0') * mulCnt;
      mulCnt *= 10;
    }
    if (isNegative) {
      conv.val = -conv.val;
    }
  } else {
    while (*pBuf) {
      if (('a' <= *pBuf) && ('f' >= *pBuf)) {
        conv.val += ((*pBuf) - 'a' + 10u) * mulCnt;
      } else if (isnumeric(*pBuf)) {
        conv.val += ((*pBuf) - '0') * mulCnt;
      } else {
        return conv;
      }
      pBuf++;
      mulCnt *= 16;
    }
  }

  conv.valid = true;
  return conv;
}

bool utilCharPrintable(const char c) {
  /* Allow any printable character plus \r and \n */
  return (((c >= 32) && (c <= 126)) || ('\r' == c) || ('\n' == c));
}

size_t utilFtoa(char *pBuf, float val) {
  size_t      charCnt    = 0;
  bool        isNegative = false;
  char *const pBase      = pBuf;

  uint16_t decimals;
  int32_t  units;

  if (val < 0.0f) {
    isNegative = true;
    val        = qfp_fmul(val, -1.0f);
  }
  decimals = qfp_float2int_z(qfp_fmul(val, 100.0f)) % 100;
  units    = qfp_float2int_z(val);

  charCnt += 3u;
  *pBuf++  = (decimals % 10) + '0';
  decimals = decimals / 10;
  *pBuf++  = (decimals % 10) + '0';
  *pBuf++  = '.';

  if (0 == units) {
    *pBuf++ = '0';
    charCnt++;
  }

  while (0 != units) {
    *pBuf++ = (units % 10) + '0';
    units   = units / 10;
    charCnt++;
  }

  if (isNegative) {
    *pBuf++ = '-';
    charCnt++;
  }

  /* Terminate and return */
  *pBuf = '\0';
  charCnt++;

  utilStrReverse(pBase, charCnt - 1u);
  return charCnt;
}

ConvFloat_t utilAtof(char *pBuf) {
  bool        isNegative = false;
  size_t      len        = 0;
  uint32_t    mulCnt     = 1u;
  uint32_t    fraction   = 0u;
  ConvFloat_t conv       = {false, 0.0f};

  if ('-' == *pBuf) {
    isNegative = true;
    pBuf++;
  }
  len = utilStrlen(pBuf);
  utilStrReverse(pBuf, len);

  while (*pBuf) {
    const char c = *pBuf++;
    /* Allow period/comma delimit, divide down if found */
    if (('.' == c) || (',' == c)) {
      fraction = mulCnt;
    } else if (isnumeric(c)) {
      const float toAdd = qfp_uint2float((c - '0') * mulCnt);
      conv.val          = qfp_fadd(conv.val, toAdd);
      mulCnt *= 10;
    } else {
      /* Invalid character found */
      return conv;
    }
  }

  if (0 != fraction) {
    conv.val = qfp_fdiv(conv.val, qfp_uint2float(fraction));
  }

  if (isNegative) {
    conv.val = qfp_fmul(conv.val, -1.0f);
  }

  conv.valid = true;
  return conv;
}

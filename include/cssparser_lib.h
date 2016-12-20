#ifndef _CSSPARSER_LIB_H_
#define _CSSPARSER_LIB_H_

#ifdef __cplusplus
extern "C"
{
#endif

    enum parse_status
    {
        RES_OK = 0,
        RES_ERR_INVALID_PARAMETER = 1,
    };

    enum parse_status parse_css(const char *css, size_t css_size);

#ifdef __cplusplus
}
#endif

#endif /* _CSSPARSER_LIB_H_ */

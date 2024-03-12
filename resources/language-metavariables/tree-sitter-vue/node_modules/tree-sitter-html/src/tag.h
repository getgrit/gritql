#include "tree_sitter/parser.h"

#include <assert.h>
#include <string.h>

typedef enum {
    AREA,
    BASE,
    BASEFONT,
    BGSOUND,
    BR,
    COL,
    COMMAND,
    EMBED,
    FRAME,
    HR,
    IMAGE,
    IMG,
    INPUT,
    ISINDEX,
    KEYGEN,
    LINK,
    MENUITEM,
    META,
    NEXTID,
    PARAM,
    SOURCE,
    TRACK,
    WBR,
    END_OF_VOID_TAGS,

    A,
    ABBR,
    ADDRESS,
    ARTICLE,
    ASIDE,
    AUDIO,
    B,
    BDI,
    BDO,
    BLOCKQUOTE,
    BODY,
    BUTTON,
    CANVAS,
    CAPTION,
    CITE,
    CODE,
    COLGROUP,
    DATA,
    DATALIST,
    DD,
    DEL,
    DETAILS,
    DFN,
    DIALOG,
    DIV,
    DL,
    DT,
    EM,
    FIELDSET,
    FIGCAPTION,
    FIGURE,
    FOOTER,
    FORM,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    HEAD,
    HEADER,
    HGROUP,
    HTML,
    I,
    IFRAME,
    INS,
    KBD,
    LABEL,
    LEGEND,
    LI,
    MAIN,
    MAP,
    MARK,
    MATH,
    MENU,
    METER,
    NAV,
    NOSCRIPT,
    OBJECT,
    OL,
    OPTGROUP,
    OPTION,
    OUTPUT,
    P,
    PICTURE,
    PRE,
    PROGRESS,
    Q,
    RB,
    RP,
    RT,
    RTC,
    RUBY,
    S,
    SAMP,
    SCRIPT,
    SECTION,
    SELECT,
    SLOT,
    SMALL,
    SPAN,
    STRONG,
    STYLE,
    SUB,
    SUMMARY,
    SUP,
    SVG,
    TABLE,
    TBODY,
    TD,
    TEMPLATE,
    TEXTAREA,
    TFOOT,
    TH,
    THEAD,
    TIME,
    TITLE,
    TR,
    U,
    UL,
    VAR,
    VIDEO,

    CUSTOM,

    END_,
} TagType;

typedef struct {
    uint32_t len;
    uint32_t cap;
    char *data;
} String;

typedef struct {
    char tag_name[16];
    TagType tag_value;
} TagMap;

typedef struct {
    TagType type;
    String custom_tag_name;
} Tag;

const TagMap TAG_TYPES_BY_TAG_NAME[126] = {
    {"AREA",       AREA      },
    {"BASE",       BASE      },
    {"BASEFONT",   BASEFONT  },
    {"BGSOUND",    BGSOUND   },
    {"BR",         BR        },
    {"COL",        COL       },
    {"COMMAND",    COMMAND   },
    {"EMBED",      EMBED     },
    {"FRAME",      FRAME     },
    {"HR",         HR        },
    {"IMAGE",      IMAGE     },
    {"IMG",        IMG       },
    {"INPUT",      INPUT     },
    {"ISINDEX",    ISINDEX   },
    {"KEYGEN",     KEYGEN    },
    {"LINK",       LINK      },
    {"MENUITEM",   MENUITEM  },
    {"META",       META      },
    {"NEXTID",     NEXTID    },
    {"PARAM",      PARAM     },
    {"SOURCE",     SOURCE    },
    {"TRACK",      TRACK     },
    {"WBR",        WBR       },
    {"A",          A         },
    {"ABBR",       ABBR      },
    {"ADDRESS",    ADDRESS   },
    {"ARTICLE",    ARTICLE   },
    {"ASIDE",      ASIDE     },
    {"AUDIO",      AUDIO     },
    {"B",          B         },
    {"BDI",        BDI       },
    {"BDO",        BDO       },
    {"BLOCKQUOTE", BLOCKQUOTE},
    {"BODY",       BODY      },
    {"BUTTON",     BUTTON    },
    {"CANVAS",     CANVAS    },
    {"CAPTION",    CAPTION   },
    {"CITE",       CITE      },
    {"CODE",       CODE      },
    {"COLGROUP",   COLGROUP  },
    {"DATA",       DATA      },
    {"DATALIST",   DATALIST  },
    {"DD",         DD        },
    {"DEL",        DEL       },
    {"DETAILS",    DETAILS   },
    {"DFN",        DFN       },
    {"DIALOG",     DIALOG    },
    {"DIV",        DIV       },
    {"DL",         DL        },
    {"DT",         DT        },
    {"EM",         EM        },
    {"FIELDSET",   FIELDSET  },
    {"FIGCAPTION", FIGCAPTION},
    {"FIGURE",     FIGURE    },
    {"FOOTER",     FOOTER    },
    {"FORM",       FORM      },
    {"H1",         H1        },
    {"H2",         H2        },
    {"H3",         H3        },
    {"H4",         H4        },
    {"H5",         H5        },
    {"H6",         H6        },
    {"HEAD",       HEAD      },
    {"HEADER",     HEADER    },
    {"HGROUP",     HGROUP    },
    {"HTML",       HTML      },
    {"I",          I         },
    {"IFRAME",     IFRAME    },
    {"INS",        INS       },
    {"KBD",        KBD       },
    {"LABEL",      LABEL     },
    {"LEGEND",     LEGEND    },
    {"LI",         LI        },
    {"MAIN",       MAIN      },
    {"MAP",        MAP       },
    {"MARK",       MARK      },
    {"MATH",       MATH      },
    {"MENU",       MENU      },
    {"METER",      METER     },
    {"NAV",        NAV       },
    {"NOSCRIPT",   NOSCRIPT  },
    {"OBJECT",     OBJECT    },
    {"OL",         OL        },
    {"OPTGROUP",   OPTGROUP  },
    {"OPTION",     OPTION    },
    {"OUTPUT",     OUTPUT    },
    {"P",          P         },
    {"PICTURE",    PICTURE   },
    {"PRE",        PRE       },
    {"PROGRESS",   PROGRESS  },
    {"Q",          Q         },
    {"RB",         RB        },
    {"RP",         RP        },
    {"RT",         RT        },
    {"RTC",        RTC       },
    {"RUBY",       RUBY      },
    {"S",          S         },
    {"SAMP",       SAMP      },
    {"SCRIPT",     SCRIPT    },
    {"SECTION",    SECTION   },
    {"SELECT",     SELECT    },
    {"SLOT",       SLOT      },
    {"SMALL",      SMALL     },
    {"SPAN",       SPAN      },
    {"STRONG",     STRONG    },
    {"STYLE",      STYLE     },
    {"SUB",        SUB       },
    {"SUMMARY",    SUMMARY   },
    {"SUP",        SUP       },
    {"SVG",        SVG       },
    {"TABLE",      TABLE     },
    {"TBODY",      TBODY     },
    {"TD",         TD        },
    {"TEMPLATE",   TEMPLATE  },
    {"TEXTAREA",   TEXTAREA  },
    {"TFOOT",      TFOOT     },
    {"TH",         TH        },
    {"THEAD",      THEAD     },
    {"TIME",       TIME      },
    {"TITLE",      TITLE     },
    {"TR",         TR        },
    {"U",          U         },
    {"UL",         UL        },
    {"VAR",        VAR       },
    {"VIDEO",      VIDEO     },
    {"CUSTOM",     CUSTOM    },
};

static const TagType TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS[] = {
    ADDRESS,  ARTICLE,    ASIDE,  BLOCKQUOTE, DETAILS, DIV, DL,
    FIELDSET, FIGCAPTION, FIGURE, FOOTER,     FORM,    H1,  H2,
    H3,       H4,         H5,     H6,         HEADER,  HR,  MAIN,
    NAV,      OL,         P,      PRE,        SECTION,
};

static TagType get_tag_from_string(const char *tag_name) {
    for (int i = 0; i < 126; i++) {
        if (strcmp(TAG_TYPES_BY_TAG_NAME[i].tag_name, tag_name) == 0) {
            return TAG_TYPES_BY_TAG_NAME[i].tag_value;
        }
    }
    return CUSTOM;
}

static inline Tag new_tag() {
    Tag tag;
    tag.type = END_;
    tag.custom_tag_name.data = NULL;
    tag.custom_tag_name.len = 0;
    tag.custom_tag_name.cap = 0;
    return tag;
}

static Tag make_tag(TagType type, const char *name) {
    Tag tag = new_tag();
    tag.type = type;
    if (type == CUSTOM) {
        tag.custom_tag_name.len = (uint32_t)strlen(name);
        tag.custom_tag_name.data =
            (char *)calloc(1, sizeof(char) * (tag.custom_tag_name.len + 1));
        strncpy(tag.custom_tag_name.data, name, tag.custom_tag_name.len);
    }
    return tag;
}

static inline void tag_free(Tag *tag) {
    if (tag->type == CUSTOM) {
        free(tag->custom_tag_name.data);
    }
    tag->custom_tag_name.data = NULL;
}

static inline bool is_void(const Tag *tag) {
    return tag->type < END_OF_VOID_TAGS;
}

static inline Tag for_name(const char *name) {
    return make_tag(get_tag_from_string(name), name);
}

static inline bool tagcmp(const Tag *_tag1, const Tag *_tag2) {
    return _tag1->type == _tag2->type &&
           (_tag1->type == CUSTOM ? strcmp(_tag1->custom_tag_name.data,
                                           _tag2->custom_tag_name.data) == 0
                                  : true);
}

static bool can_contain(Tag *self, const Tag *other) {
    TagType child = other->type;

    switch (self->type) {
        case LI:
            return child != LI;

        case DT:
        case DD:
            return child != DT && child != DD;

        case P:
            for (int i = 0; i < 26; i++) {
                if (child == TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS[i]) {
                    return false;
                }
            }
            return true;

        case COLGROUP:
            return child == COL;

        case RB:
        case RT:
        case RP:
            return child != RB && child != RT && child != RP;

        case OPTGROUP:
            return child != OPTGROUP;

        case TR:
            return child != TR;

        case TD:
        case TH:
            return child != TD && child != TH && child != TR;

        default:
            return true;
    }
}

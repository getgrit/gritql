import { FaDatabase, FaJava } from 'react-icons/fa';
import { HiOutlineCode } from 'react-icons/hi';
import type { IconType } from 'react-icons/lib';
import {
  SiCsharp,
  SiCss3,
  SiGo,
  SiHtml5,
  SiJavascript,
  SiJson,
  SiMarkdown,
  SiPhp,
  SiPython,
  SiRuby,
  SiRust,
  SiTerraform,
  SiToml,
  SiYaml,
} from 'react-icons/si';
import { DocPattern } from 'src/app/(doclike)/(default)/patterns/page';

import { Language } from '@getgrit/universal';

export const languageConfigs: {
  [key in Language]: {
    title: string;
    icon: IconType;
    color?: string;
  };
} = {
  [Language.Js]: {
    title: 'JavaScript',
    icon: SiJavascript,
    color: '#f7df1e',
  },
  [Language.Json]: {
    title: 'JSON',
    icon: SiJson,
  },
  [Language.Yaml]: {
    title: 'YAML',
    icon: SiYaml,
  },
  [Language.Sol]: {
    title: 'Solidity',
    icon: HiOutlineCode,
  },
  [Language.Css]: {
    title: 'CSS',
    icon: SiCss3,
    color: '264de4',
  },
  [Language.CSharp]: {
    title: 'C#',
    icon: SiCsharp,
  },
  [Language.Go]: {
    title: 'Go',
    icon: SiGo,
  },
  [Language.Grit]: {
    title: 'Grit',
    icon: HiOutlineCode,
  },
  [Language.Universal]: {
    title: 'Universal',
    icon: HiOutlineCode,
  },
  [Language.Hcl]: {
    title: 'Terraform (HCL)',
    icon: SiTerraform,
    color: '844FBA',
  },
  [Language.Html]: {
    title: 'HTML',
    icon: SiHtml5,
    color: 'e44d26',
  },
  [Language.Java]: {
    title: 'Java',
    icon: FaJava,
  },
  [Language.Markdown]: {
    title: 'Markdown',
    icon: SiMarkdown,
  },
  [Language.Python]: {
    title: 'Python',
    icon: SiPython,
  },
  [Language.Ruby]: {
    title: 'Ruby',
    icon: SiRuby,
  },
  [Language.Rust]: {
    title: 'Rust',
    icon: SiRust,
  },
  [Language.Sql]: {
    title: 'SQL',
    icon: FaDatabase,
  },
  [Language.Toml]: {
    title: 'TOML',
    icon: SiToml,
  },
  [Language.Php]: {
    title: 'PHP',
    icon: SiPhp,
  },
};

export const PatternLanguageButton: React.FC<{
  pattern: Pick<DocPattern, 'language'>;
  size: 'sm' | 'lg';
}> = ({ pattern, size }) => {
  const language = pattern.language ?? 'JS';
  const config = languageConfigs[language];
  if (!config) return null;
  const title = `${config.title} pattern`;

  const icon = <config.icon size={size === 'lg' ? 43 : 28} title={title} color={config.color} />;
  return language === 'JS' ? <div className='bg-white'>{icon}</div> : icon;
};

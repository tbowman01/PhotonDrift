/**
 * CliCommand Component
 * 
 * Interactive CLI command display with copy-to-clipboard functionality
 * and syntax highlighting for PhotonDrift documentation
 */

import React, { useState } from 'react';
import clsx from 'clsx';
import CodeBlock from '@theme/CodeBlock';
import styles from './styles.module.css';

interface CliCommandProps {
  command: string;
  description?: string;
  output?: string;
  showCopy?: boolean;
  language?: string;
  title?: string;
}

export default function CliCommand({
  command,
  description,
  output,
  showCopy = true,
  language = 'bash',
  title
}: CliCommandProps): JSX.Element {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(command);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy command:', err);
    }
  };

  return (
    <div className={styles.cliCommand}>
      {description && (
        <div className={styles.description}>
          {description}
        </div>
      )}
      
      <div className={styles.commandContainer}>
        <CodeBlock
          language={language}
          title={title || 'Command'}
          showLineNumbers={false}
          className={styles.commandBlock}
        >
          {command}
        </CodeBlock>
        
        {showCopy && (
          <button
            className={clsx(styles.copyButton, {
              [styles.copied]: copied
            })}
            onClick={handleCopy}
            title="Copy command"
            aria-label="Copy command to clipboard"
          >
            {copied ? '✅' : '📋'}
          </button>
        )}
      </div>
      
      {output && (
        <div className={styles.outputContainer}>
          <div className={styles.outputLabel}>Output:</div>
          <CodeBlock
            language="text"
            showLineNumbers={false}
            className={styles.outputBlock}
          >
            {output}
          </CodeBlock>
        </div>
      )}
    </div>
  );
}
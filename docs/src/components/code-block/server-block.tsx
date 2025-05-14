import type { MarkdocCodeFenceProps } from './monaco';
import { MonacoBlock } from './monaco';

// We need this wrapper to provide a server-side placeholder
export function ServerMonacoBlock(props: MarkdocCodeFenceProps) {
  return (
    <div>
      <MonacoBlock {...props} />
    </div>
  );
}

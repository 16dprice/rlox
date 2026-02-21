import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(
  context: vscode.ExtensionContext
): Promise<void> {
  const serverPath = resolveServerPath(context);

  const serverOptions: ServerOptions = {
    run: { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "rlox" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.rlox"),
    },
  };

  client = new LanguageClient(
    "rloxLanguageServer",
    "rlox Language Server",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client.start());

  context.subscriptions.push(
    vscode.commands.registerCommand("rlox.restartLanguageServer", async () => {
      if (!client) {
        return;
      }

      await client.stop();
      client.start();
      vscode.window.showInformationMessage("Rlox language server restarted.");
    })
  );
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
  }
}

function resolveServerPath(context: vscode.ExtensionContext): string {
  const configuredPath = vscode.workspace
    .getConfiguration("rlox")
    .get<string>("languageServerPath");

  if (configuredPath && configuredPath.trim().length > 0) {
    return configuredPath;
  }

  const extensionRoot = context.extensionPath;
  const repoRoot = path.resolve(extensionRoot, "..", "..");
  const binaryName = process.platform === "win32" ? "rlox-lsp.exe" : "rlox-lsp";

  return path.join(repoRoot, "tools", "rlox-lsp", "target", "debug", binaryName);
}

#define MyAppName "SpotlightSaver"
#define MyAppDescription "Windows Spotlight Saver"
#define MyAppVersion "6.0.0"
#define MyAppPublisher "Caprine Logic"
#define MyAppExeName "SpotlightSaver.exe"
#define MyAppCopyright "(c) 2022-2025 Caprine Logic"

[Setup]
AppId={{D73EBD89-4171-4B7F-8911-7E3B65CDCBEA}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppCopyright={#MyAppCopyright}
VersionInfoVersion={#MyAppVersion}
DefaultDirName={autopf}\{#MyAppPublisher}\{#MyAppName}
DisableDirPage=yes
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
AllowNoIcons=yes
LicenseFile=.\LICENSE
OutputDir=.\bin
OutputBaseFilename=SpotlightSaverSetup
SetupIconFile=.\resources\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
WizardResizable=no
ArchitecturesAllowed=x64compatible
ShowTasksTreeLines=True
AlwaysShowDirOnReadyPage=True
UninstallDisplayIcon={app}\{#MyAppExeName}
UninstallDisplayName={#MyAppName}
VersionInfoCompany={#MyAppPublisher}
VersionInfoCopyright={#MyAppCopyright}
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}
VersionInfoProductTextVersion={#MyAppVersion}
VersionInfoDescription={#MyAppDescription}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: startup; Description: "Run on Startup"

[Files]
Source: ".\bin\MinSizeRel\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs
Source: ".\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "{#MyAppName}"; ValueData: "{app}\{#MyAppExeName}"; Tasks: startup; Flags: uninsdeletevalue

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: dirifempty; Name: "{app}"

#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Possible events fired on an $(ref:automation.AutomationNode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    AccessKeyChanged = "accessKeyChanged",
    ActiveDescendantChanged = "activeDescendantChanged",
    Alert = "alert",
    ///TODO(crbug.com/1464633) Fully remove ariaAttributeChangedDeprecated starting in 122, because although it was removed in 118, it is still present in earlier versions of LaCros.
    AriaAttributeChangedDeprecated = "ariaAttributeChangedDeprecated",
    AriaCurrentChanged = "ariaCurrentChanged",
    AriaNotificationsPosted = "ariaNotificationsPosted",
    AtomicChanged = "atomicChanged",
    AutoCompleteChanged = "autoCompleteChanged",
    AutocorrectionOccured = "autocorrectionOccured",
    AutofillAvailabilityChanged = "autofillAvailabilityChanged",
    Blur = "blur",
    BusyChanged = "busyChanged",
    CaretBoundsChanged = "caretBoundsChanged",
    CheckedStateChanged = "checkedStateChanged",
    CheckedStateDescriptionChanged = "checkedStateDescriptionChanged",
    ChildrenChanged = "childrenChanged",
    Clicked = "clicked",
    Collapsed = "collapsed",
    ControlsChanged = "controlsChanged",
    DefaultActionVerbChanged = "defaultActionVerbChanged",
    DetailsChanged = "detailsChanged",
    DescribedByChanged = "describedByChanged",
    DescriptionChanged = "descriptionChanged",
    DocumentSelectionChanged = "documentSelectionChanged",
    DocumentTitleChanged = "documentTitleChanged",
    DropeffectChanged = "dropeffectChanged",
    EditableTextChanged = "editableTextChanged",
    EnabledChanged = "enabledChanged",
    EndOfTest = "endOfTest",
    Expanded = "expanded",
    ExpandedChanged = "expandedChanged",
    FlowFromChanged = "flowFromChanged",
    FlowToChanged = "flowToChanged",
    Focus = "focus",
    FocusAfterMenuClose = "focusAfterMenuClose",
    FocusChanged = "focusChanged",
    FocusContext = "focusContext",
    GrabbedChanged = "grabbedChanged",
    HaspopupChanged = "haspopupChanged",
    Hide = "hide",
    HierarchicalLevelChanged = "hierarchicalLevelChanged",
    HitTestResult = "hitTestResult",
    Hover = "hover",
    IgnoredChanged = "ignoredChanged",
    ImageAnnotationChanged = "imageAnnotationChanged",
    ImageFrameUpdated = "imageFrameUpdated",
    InvalidStatusChanged = "invalidStatusChanged",
    KeyShortcutsChanged = "keyShortcutsChanged",
    LabeledByChanged = "labeledByChanged",
    LanguageChanged = "languageChanged",
    LayoutComplete = "layoutComplete",
    LayoutInvalidated = "layoutInvalidated",
    ///fired when aria-busy goes false
    LiveRegionChanged = "liveRegionChanged",
    LiveRegionCreated = "liveRegionCreated",
    LiveRegionNodeChanged = "liveRegionNodeChanged",
    ///fired on a node within a live region.
    LiveRelevantChanged = "liveRelevantChanged",
    LiveStatusChanged = "liveStatusChanged",
    LoadComplete = "loadComplete",
    LoadStart = "loadStart",
    LocationChanged = "locationChanged",
    MediaStartedPlaying = "mediaStartedPlaying",
    MediaStoppedPlaying = "mediaStoppedPlaying",
    MenuEnd = "menuEnd",
    MenuItemSelected = "menuItemSelected",
    MenuListValueChangedDeprecated = "menuListValueChangedDeprecated",
    MenuPopupEnd = "menuPopupEnd",
    MenuPopupStart = "menuPopupStart",
    MenuStart = "menuStart",
    MouseCanceled = "mouseCanceled",
    MouseDragged = "mouseDragged",
    MouseMoved = "mouseMoved",
    MousePressed = "mousePressed",
    MouseReleased = "mouseReleased",
    MultilineStateChanged = "multilineStateChanged",
    MultiselectableStateChanged = "multiselectableStateChanged",
    NameChanged = "nameChanged",
    ObjectAttributeChanged = "objectAttributeChanged",
    OrientationChanged = "orientationChanged",
    ParentChanged = "parentChanged",
    PlaceholderChanged = "placeholderChanged",
    PositionInSetChanged = "positionInSetChanged",
    RangeValueChanged = "rangeValueChanged",
    RangeValueMaxChanged = "rangeValueMaxChanged",
    RangeValueMinChanged = "rangeValueMinChanged",
    RangeValueStepChanged = "rangeValueStepChanged",
    ReadonlyChanged = "readonlyChanged",
    RelatedNodeChanged = "relatedNodeChanged",
    RequiredStateChanged = "requiredStateChanged",
    RoleChanged = "roleChanged",
    RowCollapsed = "rowCollapsed",
    RowCountChanged = "rowCountChanged",
    RowExpanded = "rowExpanded",
    ScrollHorizontalPositionChanged = "scrollHorizontalPositionChanged",
    ScrollPositionChanged = "scrollPositionChanged",
    ScrollVerticalPositionChanged = "scrollVerticalPositionChanged",
    ScrolledToAnchor = "scrolledToAnchor",
    SelectedChanged = "selectedChanged",
    SelectedChildrenChanged = "selectedChildrenChanged",
    SelectedValueChanged = "selectedValueChanged",
    Selection = "selection",
    SelectionAdd = "selectionAdd",
    SelectionRemove = "selectionRemove",
    SetSizeChanged = "setSizeChanged",
    Show = "show",
    SortChanged = "sortChanged",
    StateChanged = "stateChanged",
    SubtreeCreated = "subtreeCreated",
    TextAttributeChanged = "textAttributeChanged",
    TextSelectionChanged = "textSelectionChanged",
    TextChanged = "textChanged",
    TooltipClosed = "tooltipClosed",
    TooltipOpened = "tooltipOpened",
    TreeChanged = "treeChanged",
    ValueInTextFieldChanged = "valueInTextFieldChanged",
    ValueChanged = "valueChanged",
    ///Deprecated.
    WindowActivated = "windowActivated",
    WindowDeactivated = "windowDeactivated",
    WindowVisibilityChanged = "windowVisibilityChanged",
}
#[wasm_bindgen]
///Describes the purpose of an $(ref:automation.AutomationNode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleType {
    Abbr = "abbr",
    Alert = "alert",
    AlertDialog = "alertDialog",
    Application = "application",
    Article = "article",
    Audio = "audio",
    Banner = "banner",
    Blockquote = "blockquote",
    Button = "button",
    Canvas = "canvas",
    Caption = "caption",
    Caret = "caret",
    Cell = "cell",
    CheckBox = "checkBox",
    Client = "client",
    Code = "code",
    ColorWell = "colorWell",
    Column = "column",
    ColumnHeader = "columnHeader",
    ComboBoxGrouping = "comboBoxGrouping",
    ComboBoxMenuButton = "comboBoxMenuButton",
    ComboBoxSelect = "comboBoxSelect",
    Comment = "comment",
    Complementary = "complementary",
    ContentDeletion = "contentDeletion",
    ContentInsertion = "contentInsertion",
    ContentInfo = "contentInfo",
    Date = "date",
    DateTime = "dateTime",
    Definition = "definition",
    DescriptionList = "descriptionList",
    DescriptionListDetailDeprecated = "descriptionListDetailDeprecated",
    DescriptionListTermDeprecated = "descriptionListTermDeprecated",
    Desktop = "desktop",
    Details = "details",
    Dialog = "dialog",
    DirectoryDeprecated = "directoryDeprecated",
    DisclosureTriangle = "disclosureTriangle",
    DisclosureTriangleGrouped = "disclosureTriangleGrouped",
    ///-------------------------------------------------------------- DPub Roles: https://www.w3.org/TR/dpub-aam-1.0/#mapping_role_table
    DocAbstract = "docAbstract",
    DocAcknowledgments = "docAcknowledgments",
    DocAfterword = "docAfterword",
    DocAppendix = "docAppendix",
    DocBackLink = "docBackLink",
    DocBiblioEntry = "docBiblioEntry",
    DocBibliography = "docBibliography",
    DocBiblioRef = "docBiblioRef",
    DocChapter = "docChapter",
    DocColophon = "docColophon",
    DocConclusion = "docConclusion",
    DocCover = "docCover",
    DocCredit = "docCredit",
    DocCredits = "docCredits",
    DocDedication = "docDedication",
    DocEndnote = "docEndnote",
    DocEndnotes = "docEndnotes",
    DocEpigraph = "docEpigraph",
    DocEpilogue = "docEpilogue",
    DocErrata = "docErrata",
    DocExample = "docExample",
    DocFootnote = "docFootnote",
    DocForeword = "docForeword",
    DocGlossary = "docGlossary",
    DocGlossRef = "docGlossRef",
    DocIndex = "docIndex",
    DocIntroduction = "docIntroduction",
    DocNoteRef = "docNoteRef",
    DocNotice = "docNotice",
    DocPageBreak = "docPageBreak",
    DocPageFooter = "docPageFooter",
    DocPageHeader = "docPageHeader",
    DocPageList = "docPageList",
    DocPart = "docPart",
    DocPreface = "docPreface",
    DocPrologue = "docPrologue",
    DocPullquote = "docPullquote",
    DocQna = "docQna",
    DocSubtitle = "docSubtitle",
    DocTip = "docTip",
    DocToc = "docToc",
    ///End DPub roles. --------------------------------------------------------------
    Document = "document",
    EmbeddedObject = "embeddedObject",
    Emphasis = "emphasis",
    Feed = "feed",
    Figcaption = "figcaption",
    Figure = "figure",
    Footer = "footer",
    Form = "form",
    GenericContainer = "genericContainer",
    ///-------------------------------------------------------------- ARIA Graphics module roles: https://rawgit.com/w3c/graphics-aam/master/#mapping_role_table
    GraphicsDocument = "graphicsDocument",
    GraphicsObject = "graphicsObject",
    GraphicsSymbol = "graphicsSymbol",
    ///End ARIA Graphics module roles. --------------------------------------------------------------
    Grid = "grid",
    GridCell = "gridCell",
    Group = "group",
    Header = "header",
    Heading = "heading",
    Iframe = "iframe",
    IframePresentational = "iframePresentational",
    Image = "image",
    ImeCandidate = "imeCandidate",
    InlineTextBox = "inlineTextBox",
    InputTime = "inputTime",
    Keyboard = "keyboard",
    LabelText = "labelText",
    LayoutTable = "layoutTable",
    LayoutTableCell = "layoutTableCell",
    LayoutTableRow = "layoutTableRow",
    Legend = "legend",
    LineBreak = "lineBreak",
    Link = "link",
    List = "list",
    ListBox = "listBox",
    ListBoxOption = "listBoxOption",
    ListGrid = "listGrid",
    ///Native
    ListItem = "listItem",
    ListMarker = "listMarker",
    Log = "log",
    Main = "main",
    Mark = "mark",
    Marquee = "marquee",
    Math = "math",
    MathMlFraction = "mathMLFraction",
    MathMlIdentifier = "mathMLIdentifier",
    MathMlMath = "mathMLMath",
    MathMlMultiscripts = "mathMLMultiscripts",
    MathMlNoneScript = "mathMLNoneScript",
    MathMlNumber = "mathMLNumber",
    MathMlOperator = "mathMLOperator",
    MathMlOver = "mathMLOver",
    MathMlPrescriptDelimiter = "mathMLPrescriptDelimiter",
    MathMlRoot = "mathMLRoot",
    MathMlRow = "mathMLRow",
    MathMlSquareRoot = "mathMLSquareRoot",
    MathMlStringLiteral = "mathMLStringLiteral",
    MathMlSub = "mathMLSub",
    MathMlSubSup = "mathMLSubSup",
    MathMlSup = "mathMLSup",
    MathMlTable = "mathMLTable",
    MathMlTableCell = "mathMLTableCell",
    MathMlTableRow = "mathMLTableRow",
    MathMlText = "mathMLText",
    MathMlUnder = "mathMLUnder",
    MathMlUnderOver = "mathMLUnderOver",
    Menu = "menu",
    MenuBar = "menuBar",
    MenuItem = "menuItem",
    MenuItemCheckBox = "menuItemCheckBox",
    MenuItemRadio = "menuItemRadio",
    MenuItemSeparator = "menuItemSeparator",
    MenuListOption = "menuListOption",
    MenuListPopup = "menuListPopup",
    Meter = "meter",
    Navigation = "navigation",
    Note = "note",
    Pane = "pane",
    Paragraph = "paragraph",
    PdfActionableHighlight = "pdfActionableHighlight",
    PdfRoot = "pdfRoot",
    PluginObject = "pluginObject",
    PopUpButton = "popUpButton",
    PortalDeprecated = "portalDeprecated",
    PreDeprecated = "preDeprecated",
    ProgressIndicator = "progressIndicator",
    RadioButton = "radioButton",
    RadioGroup = "radioGroup",
    Region = "region",
    RootWebArea = "rootWebArea",
    Row = "row",
    RowGroup = "rowGroup",
    RowHeader = "rowHeader",
    Ruby = "ruby",
    RubyAnnotation = "rubyAnnotation",
    ScrollBar = "scrollBar",
    ScrollView = "scrollView",
    Search = "search",
    SearchBox = "searchBox",
    Section = "section",
    SectionFooter = "sectionFooter",
    SectionHeader = "sectionHeader",
    SectionWithoutName = "sectionWithoutName",
    Slider = "slider",
    SpinButton = "spinButton",
    Splitter = "splitter",
    StaticText = "staticText",
    Status = "status",
    Strong = "strong",
    Subscript = "subscript",
    Suggestion = "suggestion",
    Superscript = "superscript",
    SvgRoot = "svgRoot",
    Switch = "switch",
    Tab = "tab",
    TabList = "tabList",
    TabPanel = "tabPanel",
    Table = "table",
    TableHeaderContainer = "tableHeaderContainer",
    Term = "term",
    TextField = "textField",
    TextFieldWithComboBox = "textFieldWithComboBox",
    Time = "time",
    Timer = "timer",
    TitleBar = "titleBar",
    ToggleButton = "toggleButton",
    Toolbar = "toolbar",
    Tooltip = "tooltip",
    Tree = "tree",
    TreeGrid = "treeGrid",
    TreeItem = "treeItem",
    Unknown = "unknown",
    Video = "video",
    WebView = "webView",
    Window = "window",
}
#[wasm_bindgen]
///Describes characteristics of an $(ref:automation.AutomationNode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    AutofillAvailable = "autofillAvailable",
    Collapsed = "collapsed",
    Default = "default",
    Editable = "editable",
    Expanded = "expanded",
    Focusable = "focusable",
    Focused = "focused",
    Horizontal = "horizontal",
    Hovered = "hovered",
    Ignored = "ignored",
    Invisible = "invisible",
    Linked = "linked",
    Multiline = "multiline",
    Multiselectable = "multiselectable",
    Offscreen = "offscreen",
    Protected = "protected",
    Required = "required",
    RichlyEditable = "richlyEditable",
    Vertical = "vertical",
    Visited = "visited",
    HasActions = "hasActions",
    HasInterestFor = "hasInterestFor",
}
#[wasm_bindgen]
///All possible actions that can be performed on automation nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    AnnotatePageImages = "annotatePageImages",
    Blur = "blur",
    ClearAccessibilityFocus = "clearAccessibilityFocus",
    Collapse = "collapse",
    CustomAction = "customAction",
    Decrement = "decrement",
    DoDefault = "doDefault",
    Expand = "expand",
    Focus = "focus",
    GetImageData = "getImageData",
    GetTextLocation = "getTextLocation",
    HideTooltip = "hideTooltip",
    HitTest = "hitTest",
    Increment = "increment",
    InternalInvalidateTree = "internalInvalidateTree",
    LoadInlineTextBoxes = "loadInlineTextBoxes",
    LongClick = "longClick",
    ReplaceSelectedText = "replaceSelectedText",
    RequestLayoutBasedAction = "requestLayoutBasedAction",
    ResumeMedia = "resumeMedia",
    ScrollBackward = "scrollBackward",
    ScrollDown = "scrollDown",
    ScrollForward = "scrollForward",
    ScrollLeft = "scrollLeft",
    ScrollRight = "scrollRight",
    ScrollUp = "scrollUp",
    ScrollToMakeVisible = "scrollToMakeVisible",
    ScrollToPoint = "scrollToPoint",
    ScrollToPositionAtRowColumn = "scrollToPositionAtRowColumn",
    SetAccessibilityFocus = "setAccessibilityFocus",
    SetScrollOffset = "setScrollOffset",
    SetSelection = "setSelection",
    SetSequentialFocusNavigationStartingPoint = "setSequentialFocusNavigationStartingPoint",
    SetValue = "setValue",
    ShowContextMenu = "showContextMenu",
    SignalEndOfTest = "signalEndOfTest",
    ShowTooltip = "showTooltip",
    StitchChildTree = "stitchChildTree",
    StartDuckingMedia = "startDuckingMedia",
    StopDuckingMedia = "stopDuckingMedia",
    SuspendMedia = "suspendMedia",
}
#[wasm_bindgen]
///Possible changes to the automation tree. For any given atomic change to the tree, each node that's added, removed, or changed, will appear in exactly one TreeChange, with one of these types.nodeCreated means that this node was added to the tree and its parent is new as well, so it's just one node in a new subtree that was added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeChangeType {
    ///* This node was added to the tree and its parent is new as well, so it's just one node in a new subtree that was added.
    NodeCreated = "nodeCreated",
    ///* This node was added to the tree but its parent was already in the tree, so it's possibly the root of a new subtree - it does not mean that it necessarily has children.
    SubtreeCreated = "subtreeCreated",
    ///* This node changed.
    NodeChanged = "nodeChanged",
    ///* This node's text (name) changed.
    TextChanged = "textChanged",
    ///* This node was removed.
    NodeRemoved = "nodeRemoved",
    ///* This subtree has finished an update.
    SubtreeUpdateEnd = "subtreeUpdateEnd",
}
#[wasm_bindgen]
///Where the node's name is from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameFromType {
    Attribute = "attribute",
    AttributeExplicitlyEmpty = "attributeExplicitlyEmpty",
    Caption = "caption",
    Contents = "contents",
    CssAltText = "cssAltText",
    InterestFor = "interestFor",
    Placeholder = "placeholder",
    PopoverTarget = "popoverTarget",
    Prohibited = "prohibited",
    ProhibitedAndRedundant = "prohibitedAndRedundant",
    RelatedElement = "relatedElement",
    Title = "title",
    Value = "value",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptionFromType {
    AriaDescription = "ariaDescription",
    AttributeExplicitlyEmpty = "attributeExplicitlyEmpty",
    ButtonLabel = "buttonLabel",
    InterestFor = "interestFor",
    PopoverTarget = "popoverTarget",
    ProhibitedNameRepair = "prohibitedNameRepair",
    RelatedElement = "relatedElement",
    RubyAnnotation = "rubyAnnotation",
    Summary = "summary",
    SvgDescElement = "svgDescElement",
    TableCaption = "tableCaption",
    Title = "title",
}
#[wasm_bindgen]
///The input restriction for a object -- even non-controls can be disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Restriction {
    Disabled = "disabled",
    ReadOnly = "readOnly",
}
#[wasm_bindgen]
///Availability and types for an interactive popup element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HasPopup {
    False = "false",
    True = "true",
    Menu = "menu",
    Listbox = "listbox",
    Tree = "tree",
    Grid = "grid",
    Dialog = "dialog",
}
#[wasm_bindgen]
///Indicates the ARIA-current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaCurrentState {
    False = "false",
    True = "true",
    Page = "page",
    Step = "step",
    Location = "location",
    Date = "date",
    Time = "time",
}
#[wasm_bindgen]
///Lists the values that `invalidState` can take on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidState {
    False = "false",
    True = "true",
}
#[wasm_bindgen]
///Describes possible actions when performing a do default action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultActionVerb {
    Activate = "activate",
    Check = "check",
    Click = "click",
    ClickAncestor = "clickAncestor",
    ClickInHitTest = "clickInHitTest",
    ClickNotInHitTest = "clickNotInHitTest",
    Jump = "jump",
    Open = "open",
    Press = "press",
    Select = "select",
    Uncheck = "uncheck",
}
#[wasm_bindgen]
///Types of markers on text. See AutomationNode.markerTypes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Spelling = "spelling",
    Grammar = "grammar",
    TextMatch = "textMatch",
    ActiveSuggestion = "activeSuggestion",
    Suggestion = "suggestion",
    Highlight = "highlight",
}
#[wasm_bindgen]
///A command associated with an $(ref:automation.AutomationIntent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentCommandType {
    ClearSelection = "clearSelection",
    Delete = "delete",
    Dictate = "dictate",
    ExtendSelection = "extendSelection",
    Format = "format",
    History = "history",
    Insert = "insert",
    Marker = "marker",
    MoveSelection = "moveSelection",
    SetSelection = "setSelection",
}
#[wasm_bindgen]
///The type of an input event associated with an $(ref:automation.AutomationIntent). It describes an edit command, e.g. IntentCommandType.insert, in more detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentInputEventType {
    ///Insertion.
    InsertText = "insertText",
    InsertLineBreak = "insertLineBreak",
    InsertParagraph = "insertParagraph",
    InsertOrderedList = "insertOrderedList",
    InsertUnorderedList = "insertUnorderedList",
    InsertHorizontalRule = "insertHorizontalRule",
    InsertFromPaste = "insertFromPaste",
    InsertFromDrop = "insertFromDrop",
    InsertFromYank = "insertFromYank",
    InsertTranspose = "insertTranspose",
    InsertReplacementText = "insertReplacementText",
    InsertCompositionText = "insertCompositionText",
    InsertLink = "insertLink",
    ///Deletion.
    DeleteWordBackward = "deleteWordBackward",
    DeleteWordForward = "deleteWordForward",
    DeleteSoftLineBackward = "deleteSoftLineBackward",
    DeleteSoftLineForward = "deleteSoftLineForward",
    DeleteHardLineBackward = "deleteHardLineBackward",
    DeleteHardLineForward = "deleteHardLineForward",
    DeleteContentBackward = "deleteContentBackward",
    DeleteContentForward = "deleteContentForward",
    DeleteByCut = "deleteByCut",
    DeleteByDrag = "deleteByDrag",
    ///History.
    HistoryUndo = "historyUndo",
    HistoryRedo = "historyRedo",
    ///Formatting.
    FormatBold = "formatBold",
    FormatItalic = "formatItalic",
    FormatUnderline = "formatUnderline",
    FormatStrikeThrough = "formatStrikeThrough",
    FormatSuperscript = "formatSuperscript",
    FormatSubscript = "formatSubscript",
    FormatJustifyCenter = "formatJustifyCenter",
    FormatJustifyFull = "formatJustifyFull",
    FormatJustifyRight = "formatJustifyRight",
    FormatJustifyLeft = "formatJustifyLeft",
    FormatIndent = "formatIndent",
    FormatOutdent = "formatOutdent",
    FormatRemove = "formatRemove",
    FormatSetBlockTextDirection = "formatSetBlockTextDirection",
}
#[wasm_bindgen]
///A text boundary associated with an $(ref:automation.AutomationIntent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentTextBoundaryType {
    Character = "character",
    FormatEnd = "formatEnd",
    FormatStart = "formatStart",
    FormatStartOrEnd = "formatStartOrEnd",
    LineEnd = "lineEnd",
    LineStart = "lineStart",
    LineStartOrEnd = "lineStartOrEnd",
    Object = "object",
    PageEnd = "pageEnd",
    PageStart = "pageStart",
    PageStartOrEnd = "pageStartOrEnd",
    ParagraphEnd = "paragraphEnd",
    ParagraphStart = "paragraphStart",
    ParagraphStartSkippingEmptyParagraphs = "paragraphStartSkippingEmptyParagraphs",
    ParagraphStartOrEnd = "paragraphStartOrEnd",
    SentenceEnd = "sentenceEnd",
    SentenceStart = "sentenceStart",
    SentenceStartOrEnd = "sentenceStartOrEnd",
    WebPage = "webPage",
    WordEnd = "wordEnd",
    WordStart = "wordStart",
    WordStartOrEnd = "wordStartOrEnd",
}
#[wasm_bindgen]
///A move direction associated with an $(ref:automation.AutomationIntent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentMoveDirectionType {
    Backward = "backward",
    Forward = "forward",
}
#[wasm_bindgen]
///A sort applied to a table row or column header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirectionType {
    Unsorted = "unsorted",
    Ascending = "ascending",
    Descending = "descending",
    Other = "other",
}
#[wasm_bindgen]
///A type of AutomationPosition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionType {
    Null = "null",
    Text = "text",
    Tree = "tree",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Rect")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Rect;
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &Rect) -> i32;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &Rect, val: i32);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &Rect) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &Rect, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &Rect) -> i32;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &Rect, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Rect) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Rect, val: i32);
}
impl Rect {
    ///Construct a new `Rect`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FindParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FindParams;
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &FindParams) -> Option<Object>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &FindParams, val: &Object);
    ///Get the `attributes` field of this object.
    #[wasm_bindgen(method, getter = "attributes")]
    pub fn get_attributes(this: &FindParams) -> Option<Object>;
    ///Change the `attributes` field of this object.
    #[wasm_bindgen(method, setter = "attributes")]
    pub fn set_attributes(this: &FindParams, val: &Object);
    ///Get the `role` field of this object.
    #[wasm_bindgen(method, getter = "role")]
    pub fn get_role(this: &FindParams) -> Option<RoleType>;
    ///Change the `role` field of this object.
    #[wasm_bindgen(method, setter = "role")]
    pub fn set_role(this: &FindParams, val: RoleType);
}
impl FindParams {
    ///Construct a new `FindParams`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &Object) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_attributes()` instead."]
    pub fn attributes(&mut self, val: &Object) -> &mut Self {
        self.set_attributes(val);
        self
    }
    #[deprecated = "Use `set_role()` instead."]
    pub fn role(&mut self, val: RoleType) -> &mut Self {
        self.set_role(val);
        self
    }
}
impl Default for FindParams {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetDocumentSelectionParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetDocumentSelectionParams;
    ///Get the `anchorObject` field of this object.
    #[wasm_bindgen(method, getter = "anchorObject")]
    pub fn get_anchor_object(this: &SetDocumentSelectionParams) -> Object;
    ///Change the `anchorObject` field of this object.
    #[wasm_bindgen(method, setter = "anchorObject")]
    pub fn set_anchor_object(this: &SetDocumentSelectionParams, val: &Object);
    ///Get the `anchorOffset` field of this object.
    #[wasm_bindgen(method, getter = "anchorOffset")]
    pub fn get_anchor_offset(this: &SetDocumentSelectionParams) -> i32;
    ///Change the `anchorOffset` field of this object.
    #[wasm_bindgen(method, setter = "anchorOffset")]
    pub fn set_anchor_offset(this: &SetDocumentSelectionParams, val: i32);
    ///Get the `focusObject` field of this object.
    #[wasm_bindgen(method, getter = "focusObject")]
    pub fn get_focus_object(this: &SetDocumentSelectionParams) -> Object;
    ///Change the `focusObject` field of this object.
    #[wasm_bindgen(method, setter = "focusObject")]
    pub fn set_focus_object(this: &SetDocumentSelectionParams, val: &Object);
    ///Get the `focusOffset` field of this object.
    #[wasm_bindgen(method, getter = "focusOffset")]
    pub fn get_focus_offset(this: &SetDocumentSelectionParams) -> i32;
    ///Change the `focusOffset` field of this object.
    #[wasm_bindgen(method, setter = "focusOffset")]
    pub fn set_focus_offset(this: &SetDocumentSelectionParams, val: i32);
}
impl SetDocumentSelectionParams {
    ///Construct a new `SetDocumentSelectionParams`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_anchor_object()` instead."]
    pub fn anchor_object(&mut self, val: &Object) -> &mut Self {
        self.set_anchor_object(val);
        self
    }
    #[deprecated = "Use `set_anchor_offset()` instead."]
    pub fn anchor_offset(&mut self, val: i32) -> &mut Self {
        self.set_anchor_offset(val);
        self
    }
    #[deprecated = "Use `set_focus_object()` instead."]
    pub fn focus_object(&mut self, val: &Object) -> &mut Self {
        self.set_focus_object(val);
        self
    }
    #[deprecated = "Use `set_focus_offset()` instead."]
    pub fn focus_offset(&mut self, val: i32) -> &mut Self {
        self.set_focus_offset(val);
        self
    }
}
impl Default for SetDocumentSelectionParams {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AutomationIntent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AutomationIntent;
    ///Get the `command` field of this object.
    #[wasm_bindgen(method, getter = "command")]
    pub fn get_command(this: &AutomationIntent) -> IntentCommandType;
    ///Change the `command` field of this object.
    #[wasm_bindgen(method, setter = "command")]
    pub fn set_command(this: &AutomationIntent, val: IntentCommandType);
    ///Get the `textBoundary` field of this object.
    #[wasm_bindgen(method, getter = "textBoundary")]
    pub fn get_text_boundary(this: &AutomationIntent) -> IntentTextBoundaryType;
    ///Change the `textBoundary` field of this object.
    #[wasm_bindgen(method, setter = "textBoundary")]
    pub fn set_text_boundary(this: &AutomationIntent, val: IntentTextBoundaryType);
    ///Get the `moveDirection` field of this object.
    #[wasm_bindgen(method, getter = "moveDirection")]
    pub fn get_move_direction(this: &AutomationIntent) -> Option<IntentMoveDirectionType>;
    ///Change the `moveDirection` field of this object.
    #[wasm_bindgen(method, setter = "moveDirection")]
    pub fn set_move_direction(this: &AutomationIntent, val: IntentMoveDirectionType);
}
impl AutomationIntent {
    ///Construct a new `AutomationIntent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_command()` instead."]
    pub fn command(&mut self, val: IntentCommandType) -> &mut Self {
        self.set_command(val);
        self
    }
    #[deprecated = "Use `set_text_boundary()` instead."]
    pub fn text_boundary(&mut self, val: IntentTextBoundaryType) -> &mut Self {
        self.set_text_boundary(val);
        self
    }
    #[deprecated = "Use `set_move_direction()` instead."]
    pub fn move_direction(&mut self, val: IntentMoveDirectionType) -> &mut Self {
        self.set_move_direction(val);
        self
    }
}
impl Default for AutomationIntent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AutomationEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AutomationEvent;
    ///Get the `target` field of this object.
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &AutomationEvent) -> AutomationNode;
    ///Change the `target` field of this object.
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &AutomationEvent, val: &AutomationNode);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &AutomationEvent) -> EventType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &AutomationEvent, val: EventType);
    ///Get the `mouseY` field of this object.
    #[wasm_bindgen(method, getter = "mouseY")]
    pub fn get_mouse_y(this: &AutomationEvent) -> i32;
    ///Change the `mouseY` field of this object.
    #[wasm_bindgen(method, setter = "mouseY")]
    pub fn set_mouse_y(this: &AutomationEvent, val: i32);
    ///Get the `intents` field of this object.
    #[wasm_bindgen(method, getter = "intents")]
    pub fn get_intents(this: &AutomationEvent) -> Array;
    ///Change the `intents` field of this object.
    #[wasm_bindgen(method, setter = "intents")]
    pub fn set_intents(this: &AutomationEvent, val: &Array);
    ///Get the `eventFrom` field of this object.
    #[wasm_bindgen(method, getter = "eventFrom")]
    pub fn get_event_from(this: &AutomationEvent) -> String;
    ///Change the `eventFrom` field of this object.
    #[wasm_bindgen(method, setter = "eventFrom")]
    pub fn set_event_from(this: &AutomationEvent, val: String);
    ///Get the `stopPropagation` field of this object.
    #[wasm_bindgen(method, getter = "stopPropagation")]
    pub fn get_stop_propagation(this: &AutomationEvent) -> Function;
    ///Change the `stopPropagation` field of this object.
    #[wasm_bindgen(method, setter = "stopPropagation")]
    pub fn set_stop_propagation(this: &AutomationEvent, val: &Function);
    ///Get the `mouseX` field of this object.
    #[wasm_bindgen(method, getter = "mouseX")]
    pub fn get_mouse_x(this: &AutomationEvent) -> i32;
    ///Change the `mouseX` field of this object.
    #[wasm_bindgen(method, setter = "mouseX")]
    pub fn set_mouse_x(this: &AutomationEvent, val: i32);
}
impl AutomationEvent {
    ///Construct a new `AutomationEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_target()` instead."]
    pub fn target(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_target(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: EventType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_mouse_y()` instead."]
    pub fn mouse_y(&mut self, val: i32) -> &mut Self {
        self.set_mouse_y(val);
        self
    }
    #[deprecated = "Use `set_intents()` instead."]
    pub fn intents(&mut self, val: &Array) -> &mut Self {
        self.set_intents(val);
        self
    }
    #[deprecated = "Use `set_event_from()` instead."]
    pub fn event_from(&mut self, val: String) -> &mut Self {
        self.set_event_from(val);
        self
    }
    #[deprecated = "Use `set_stop_propagation()` instead."]
    pub fn stop_propagation(&mut self, val: &Function) -> &mut Self {
        self.set_stop_propagation(val);
        self
    }
    #[deprecated = "Use `set_mouse_x()` instead."]
    pub fn mouse_x(&mut self, val: i32) -> &mut Self {
        self.set_mouse_x(val);
        self
    }
}
impl Default for AutomationEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TreeChange")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TreeChange;
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &TreeChange) -> TreeChangeType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &TreeChange, val: TreeChangeType);
    ///Get the `target` field of this object.
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &TreeChange) -> AutomationNode;
    ///Change the `target` field of this object.
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &TreeChange, val: &AutomationNode);
}
impl TreeChange {
    ///Construct a new `TreeChange`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: TreeChangeType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_target()` instead."]
    pub fn target(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_target(val);
        self
    }
}
impl Default for TreeChange {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Possible tree changes to listen to using addTreeChangeObserver. Note that listening to all tree changes can be expensive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeChangeObserverFilter {
    NoTreeChanges = "noTreeChanges",
    LiveRegionTreeChanges = "liveRegionTreeChanges",
    TextMarkerChanges = "textMarkerChanges",
    AllTreeChanges = "allTreeChanges",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CustomAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CustomAction;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CustomAction) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CustomAction, val: i32);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &CustomAction) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &CustomAction, val: String);
}
impl CustomAction {
    ///Construct a new `CustomAction`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
}
impl Default for CustomAction {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Marker")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Marker;
    ///Get the `endOffset` field of this object.
    #[wasm_bindgen(method, getter = "endOffset")]
    pub fn get_end_offset(this: &Marker) -> i32;
    ///Change the `endOffset` field of this object.
    #[wasm_bindgen(method, setter = "endOffset")]
    pub fn set_end_offset(this: &Marker, val: i32);
    ///Get the `startOffset` field of this object.
    #[wasm_bindgen(method, getter = "startOffset")]
    pub fn get_start_offset(this: &Marker) -> i32;
    ///Change the `startOffset` field of this object.
    #[wasm_bindgen(method, setter = "startOffset")]
    pub fn set_start_offset(this: &Marker, val: i32);
    ///Get the `flags` field of this object.
    #[wasm_bindgen(method, getter = "flags")]
    pub fn get_flags(this: &Marker) -> Object;
    ///Change the `flags` field of this object.
    #[wasm_bindgen(method, setter = "flags")]
    pub fn set_flags(this: &Marker, val: &Object);
}
impl Marker {
    ///Construct a new `Marker`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_end_offset()` instead."]
    pub fn end_offset(&mut self, val: i32) -> &mut Self {
        self.set_end_offset(val);
        self
    }
    #[deprecated = "Use `set_start_offset()` instead."]
    pub fn start_offset(&mut self, val: i32) -> &mut Self {
        self.set_start_offset(val);
        self
    }
    #[deprecated = "Use `set_flags()` instead."]
    pub fn flags(&mut self, val: &Object) -> &mut Self {
        self.set_flags(val);
        self
    }
}
impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AutomationPosition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AutomationPosition;
    ///Get the `atStartOfDocument` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfDocument")]
    pub fn get_at_start_of_document(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfDocument` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfDocument")]
    pub fn set_at_start_of_document(this: &AutomationPosition, val: &Function);
    ///Get the `atStartOfWord` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfWord")]
    pub fn get_at_start_of_word(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfWord` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfWord")]
    pub fn set_at_start_of_word(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextParagraphStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextParagraphStartPosition")]
    pub fn get_move_to_next_paragraph_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextParagraphStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextParagraphStartPosition")]
    pub fn set_move_to_next_paragraph_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousLeafTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousLeafTextPosition")]
    pub fn get_move_to_previous_leaf_text_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousLeafTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousLeafTextPosition")]
    pub fn set_move_to_previous_leaf_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextLineStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextLineStartPosition")]
    pub fn get_move_to_next_line_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextLineStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextLineStartPosition")]
    pub fn set_move_to_next_line_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `maxTextOffset` field of this object.
    #[wasm_bindgen(method, getter = "maxTextOffset")]
    pub fn get_max_text_offset(this: &AutomationPosition) -> Function;
    ///Change the `maxTextOffset` field of this object.
    #[wasm_bindgen(method, setter = "maxTextOffset")]
    pub fn set_max_text_offset(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextLeafTreePosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextLeafTreePosition")]
    pub fn get_move_to_next_leaf_tree_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextLeafTreePosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextLeafTreePosition")]
    pub fn set_move_to_next_leaf_tree_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPositionAtEndOfAnchor` field of this object.
    #[wasm_bindgen(method, getter = "moveToPositionAtEndOfAnchor")]
    pub fn get_move_to_position_at_end_of_anchor(this: &AutomationPosition) -> Function;
    ///Change the `moveToPositionAtEndOfAnchor` field of this object.
    #[wasm_bindgen(method, setter = "moveToPositionAtEndOfAnchor")]
    pub fn set_move_to_position_at_end_of_anchor(this: &AutomationPosition, val: &Function);
    ///Get the `childIndex` field of this object.
    #[wasm_bindgen(method, getter = "childIndex")]
    pub fn get_child_index(this: &AutomationPosition) -> i32;
    ///Change the `childIndex` field of this object.
    #[wasm_bindgen(method, setter = "childIndex")]
    pub fn set_child_index(this: &AutomationPosition, val: i32);
    ///Get the `atEndOfFormat` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfFormat")]
    pub fn get_at_end_of_format(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfFormat` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfFormat")]
    pub fn set_at_end_of_format(this: &AutomationPosition, val: &Function);
    ///Get the `isInWhiteSpace` field of this object.
    #[wasm_bindgen(method, getter = "isInWhiteSpace")]
    pub fn get_is_in_white_space(this: &AutomationPosition) -> Function;
    ///Change the `isInWhiteSpace` field of this object.
    #[wasm_bindgen(method, setter = "isInWhiteSpace")]
    pub fn set_is_in_white_space(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousPageEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousPageEndPosition")]
    pub fn get_move_to_previous_page_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousPageEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousPageEndPosition")]
    pub fn set_move_to_previous_page_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousParagraphEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousParagraphEndPosition")]
    pub fn get_move_to_previous_paragraph_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousParagraphEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousParagraphEndPosition")]
    pub fn set_move_to_previous_paragraph_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `asTreePosition` field of this object.
    #[wasm_bindgen(method, getter = "asTreePosition")]
    pub fn get_as_tree_position(this: &AutomationPosition) -> Function;
    ///Change the `asTreePosition` field of this object.
    #[wasm_bindgen(method, setter = "asTreePosition")]
    pub fn set_as_tree_position(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfDocument` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfDocument")]
    pub fn get_at_end_of_document(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfDocument` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfDocument")]
    pub fn set_at_end_of_document(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPositionAtEndOfDocument` field of this object.
    #[wasm_bindgen(method, getter = "moveToPositionAtEndOfDocument")]
    pub fn get_move_to_position_at_end_of_document(this: &AutomationPosition) -> Function;
    ///Change the `moveToPositionAtEndOfDocument` field of this object.
    #[wasm_bindgen(method, setter = "moveToPositionAtEndOfDocument")]
    pub fn set_move_to_position_at_end_of_document(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousLineStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousLineStartPosition")]
    pub fn get_move_to_previous_line_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousLineStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousLineStartPosition")]
    pub fn set_move_to_previous_line_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextFormatStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextFormatStartPosition")]
    pub fn get_move_to_next_format_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextFormatStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextFormatStartPosition")]
    pub fn set_move_to_next_format_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `isNullPosition` field of this object.
    #[wasm_bindgen(method, getter = "isNullPosition")]
    pub fn get_is_null_position(this: &AutomationPosition) -> Function;
    ///Change the `isNullPosition` field of this object.
    #[wasm_bindgen(method, setter = "isNullPosition")]
    pub fn set_is_null_position(this: &AutomationPosition, val: &Function);
    ///Get the `atStartOfLine` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfLine")]
    pub fn get_at_start_of_line(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfLine` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfLine")]
    pub fn set_at_start_of_line(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousFormatEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousFormatEndPosition")]
    pub fn get_move_to_previous_format_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousFormatEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousFormatEndPosition")]
    pub fn set_move_to_previous_format_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousFormatStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousFormatStartPosition")]
    pub fn get_move_to_previous_format_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousFormatStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousFormatStartPosition")]
    pub fn set_move_to_previous_format_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `isLeafTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "isLeafTextPosition")]
    pub fn get_is_leaf_text_position(this: &AutomationPosition) -> Function;
    ///Change the `isLeafTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "isLeafTextPosition")]
    pub fn set_is_leaf_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToParentPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToParentPosition")]
    pub fn get_move_to_parent_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToParentPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToParentPosition")]
    pub fn set_move_to_parent_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextLeafTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextLeafTextPosition")]
    pub fn get_move_to_next_leaf_text_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextLeafTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextLeafTextPosition")]
    pub fn set_move_to_next_leaf_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfLine` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfLine")]
    pub fn get_at_end_of_line(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfLine` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfLine")]
    pub fn set_at_end_of_line(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfWord` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfWord")]
    pub fn get_at_end_of_word(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfWord` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfWord")]
    pub fn set_at_end_of_word(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextParagraphEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextParagraphEndPosition")]
    pub fn get_move_to_next_paragraph_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextParagraphEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextParagraphEndPosition")]
    pub fn set_move_to_next_paragraph_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextCharacterPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextCharacterPosition")]
    pub fn get_move_to_next_character_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextCharacterPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextCharacterPosition")]
    pub fn set_move_to_next_character_position(this: &AutomationPosition, val: &Function);
    ///Get the `isTreePosition` field of this object.
    #[wasm_bindgen(method, getter = "isTreePosition")]
    pub fn get_is_tree_position(this: &AutomationPosition) -> Function;
    ///Change the `isTreePosition` field of this object.
    #[wasm_bindgen(method, setter = "isTreePosition")]
    pub fn set_is_tree_position(this: &AutomationPosition, val: &Function);
    ///Get the `getText` field of this object.
    #[wasm_bindgen(method, getter = "getText")]
    pub fn get_get_text(this: &AutomationPosition) -> Function;
    ///Change the `getText` field of this object.
    #[wasm_bindgen(method, setter = "getText")]
    pub fn set_get_text(this: &AutomationPosition, val: &Function);
    ///Get the `asLeafTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "asLeafTextPosition")]
    pub fn get_as_leaf_text_position(this: &AutomationPosition) -> Function;
    ///Change the `asLeafTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "asLeafTextPosition")]
    pub fn set_as_leaf_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousPageStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousPageStartPosition")]
    pub fn get_move_to_previous_page_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousPageStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousPageStartPosition")]
    pub fn set_move_to_previous_page_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextLineEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextLineEndPosition")]
    pub fn get_move_to_next_line_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextLineEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextLineEndPosition")]
    pub fn set_move_to_next_line_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `asTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "asTextPosition")]
    pub fn get_as_text_position(this: &AutomationPosition) -> Function;
    ///Change the `asTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "asTextPosition")]
    pub fn set_as_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextWordStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextWordStartPosition")]
    pub fn get_move_to_next_word_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextWordStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextWordStartPosition")]
    pub fn set_move_to_next_word_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `isValid` field of this object.
    #[wasm_bindgen(method, getter = "isValid")]
    pub fn get_is_valid(this: &AutomationPosition) -> Function;
    ///Change the `isValid` field of this object.
    #[wasm_bindgen(method, setter = "isValid")]
    pub fn set_is_valid(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousWordEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousWordEndPosition")]
    pub fn get_move_to_previous_word_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousWordEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousWordEndPosition")]
    pub fn set_move_to_previous_word_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPositionAtStartOfDocument` field of this object.
    #[wasm_bindgen(method, getter = "moveToPositionAtStartOfDocument")]
    pub fn get_move_to_position_at_start_of_document(this: &AutomationPosition) -> Function;
    ///Change the `moveToPositionAtStartOfDocument` field of this object.
    #[wasm_bindgen(method, setter = "moveToPositionAtStartOfDocument")]
    pub fn set_move_to_position_at_start_of_document(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfAnchor` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfAnchor")]
    pub fn get_at_end_of_anchor(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfAnchor` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfAnchor")]
    pub fn set_at_end_of_anchor(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfParagraph` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfParagraph")]
    pub fn get_at_end_of_paragraph(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfParagraph` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfParagraph")]
    pub fn set_at_end_of_paragraph(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousLeafTreePosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousLeafTreePosition")]
    pub fn get_move_to_previous_leaf_tree_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousLeafTreePosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousLeafTreePosition")]
    pub fn set_move_to_previous_leaf_tree_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousParagraphStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousParagraphStartPosition")]
    pub fn get_move_to_previous_paragraph_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousParagraphStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousParagraphStartPosition")]
    pub fn set_move_to_previous_paragraph_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextFormatEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextFormatEndPosition")]
    pub fn get_move_to_next_format_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextFormatEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextFormatEndPosition")]
    pub fn set_move_to_next_format_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `affinity` field of this object.
    #[wasm_bindgen(method, getter = "affinity")]
    pub fn get_affinity(this: &AutomationPosition) -> String;
    ///Change the `affinity` field of this object.
    #[wasm_bindgen(method, setter = "affinity")]
    pub fn set_affinity(this: &AutomationPosition, val: String);
    ///Get the `isTextPosition` field of this object.
    #[wasm_bindgen(method, getter = "isTextPosition")]
    pub fn get_is_text_position(this: &AutomationPosition) -> Function;
    ///Change the `isTextPosition` field of this object.
    #[wasm_bindgen(method, setter = "isTextPosition")]
    pub fn set_is_text_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousAnchorPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousAnchorPosition")]
    pub fn get_move_to_previous_anchor_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousAnchorPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousAnchorPosition")]
    pub fn set_move_to_previous_anchor_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextWordEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextWordEndPosition")]
    pub fn get_move_to_next_word_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextWordEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextWordEndPosition")]
    pub fn set_move_to_next_word_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextPageEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextPageEndPosition")]
    pub fn get_move_to_next_page_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextPageEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextPageEndPosition")]
    pub fn set_move_to_next_page_end_position(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousWordStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousWordStartPosition")]
    pub fn get_move_to_previous_word_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousWordStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousWordStartPosition")]
    pub fn set_move_to_previous_word_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `textOffset` field of this object.
    #[wasm_bindgen(method, getter = "textOffset")]
    pub fn get_text_offset(this: &AutomationPosition) -> i32;
    ///Change the `textOffset` field of this object.
    #[wasm_bindgen(method, setter = "textOffset")]
    pub fn set_text_offset(this: &AutomationPosition, val: i32);
    ///Get the `node` field of this object.
    #[wasm_bindgen(method, getter = "node")]
    pub fn get_node(this: &AutomationPosition) -> Option<AutomationNode>;
    ///Change the `node` field of this object.
    #[wasm_bindgen(method, setter = "node")]
    pub fn set_node(this: &AutomationPosition, val: &AutomationNode);
    ///Get the `atStartOfParagraph` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfParagraph")]
    pub fn get_at_start_of_paragraph(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfParagraph` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfParagraph")]
    pub fn set_at_start_of_paragraph(this: &AutomationPosition, val: &Function);
    ///Get the `isInTextObject` field of this object.
    #[wasm_bindgen(method, getter = "isInTextObject")]
    pub fn get_is_in_text_object(this: &AutomationPosition) -> Function;
    ///Change the `isInTextObject` field of this object.
    #[wasm_bindgen(method, setter = "isInTextObject")]
    pub fn set_is_in_text_object(this: &AutomationPosition, val: &Function);
    ///Get the `atStartOfFormat` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfFormat")]
    pub fn get_at_start_of_format(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfFormat` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfFormat")]
    pub fn set_at_start_of_format(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousCharacterPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousCharacterPosition")]
    pub fn get_move_to_previous_character_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousCharacterPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousCharacterPosition")]
    pub fn set_move_to_previous_character_position(this: &AutomationPosition, val: &Function);
    ///Get the `atStartOfPage` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfPage")]
    pub fn get_at_start_of_page(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfPage` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfPage")]
    pub fn set_at_start_of_page(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPositionAtStartOfAnchor` field of this object.
    #[wasm_bindgen(method, getter = "moveToPositionAtStartOfAnchor")]
    pub fn get_move_to_position_at_start_of_anchor(this: &AutomationPosition) -> Function;
    ///Change the `moveToPositionAtStartOfAnchor` field of this object.
    #[wasm_bindgen(method, setter = "moveToPositionAtStartOfAnchor")]
    pub fn set_move_to_position_at_start_of_anchor(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextAnchorPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextAnchorPosition")]
    pub fn get_move_to_next_anchor_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextAnchorPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextAnchorPosition")]
    pub fn set_move_to_next_anchor_position(this: &AutomationPosition, val: &Function);
    ///Get the `atEndOfPage` field of this object.
    #[wasm_bindgen(method, getter = "atEndOfPage")]
    pub fn get_at_end_of_page(this: &AutomationPosition) -> Function;
    ///Change the `atEndOfPage` field of this object.
    #[wasm_bindgen(method, setter = "atEndOfPage")]
    pub fn set_at_end_of_page(this: &AutomationPosition, val: &Function);
    ///Get the `moveToNextPageStartPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToNextPageStartPosition")]
    pub fn get_move_to_next_page_start_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToNextPageStartPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToNextPageStartPosition")]
    pub fn set_move_to_next_page_start_position(this: &AutomationPosition, val: &Function);
    ///Get the `isInLineBreak` field of this object.
    #[wasm_bindgen(method, getter = "isInLineBreak")]
    pub fn get_is_in_line_break(this: &AutomationPosition) -> Function;
    ///Change the `isInLineBreak` field of this object.
    #[wasm_bindgen(method, setter = "isInLineBreak")]
    pub fn set_is_in_line_break(this: &AutomationPosition, val: &Function);
    ///Get the `atStartOfAnchor` field of this object.
    #[wasm_bindgen(method, getter = "atStartOfAnchor")]
    pub fn get_at_start_of_anchor(this: &AutomationPosition) -> Function;
    ///Change the `atStartOfAnchor` field of this object.
    #[wasm_bindgen(method, setter = "atStartOfAnchor")]
    pub fn set_at_start_of_anchor(this: &AutomationPosition, val: &Function);
    ///Get the `moveToPreviousLineEndPosition` field of this object.
    #[wasm_bindgen(method, getter = "moveToPreviousLineEndPosition")]
    pub fn get_move_to_previous_line_end_position(this: &AutomationPosition) -> Function;
    ///Change the `moveToPreviousLineEndPosition` field of this object.
    #[wasm_bindgen(method, setter = "moveToPreviousLineEndPosition")]
    pub fn set_move_to_previous_line_end_position(this: &AutomationPosition, val: &Function);
}
impl AutomationPosition {
    ///Construct a new `AutomationPosition`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_at_start_of_document()` instead."]
    pub fn at_start_of_document(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_document(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_word()` instead."]
    pub fn at_start_of_word(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_word(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_paragraph_start_position()` instead."]
    pub fn move_to_next_paragraph_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_paragraph_start_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_leaf_text_position()` instead."]
    pub fn move_to_previous_leaf_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_leaf_text_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_line_start_position()` instead."]
    pub fn move_to_next_line_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_line_start_position(val);
        self
    }
    #[deprecated = "Use `set_max_text_offset()` instead."]
    pub fn max_text_offset(&mut self, val: &Function) -> &mut Self {
        self.set_max_text_offset(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_leaf_tree_position()` instead."]
    pub fn move_to_next_leaf_tree_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_leaf_tree_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_position_at_end_of_anchor()` instead."]
    pub fn move_to_position_at_end_of_anchor(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_position_at_end_of_anchor(val);
        self
    }
    #[deprecated = "Use `set_child_index()` instead."]
    pub fn child_index(&mut self, val: i32) -> &mut Self {
        self.set_child_index(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_format()` instead."]
    pub fn at_end_of_format(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_format(val);
        self
    }
    #[deprecated = "Use `set_is_in_white_space()` instead."]
    pub fn is_in_white_space(&mut self, val: &Function) -> &mut Self {
        self.set_is_in_white_space(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_page_end_position()` instead."]
    pub fn move_to_previous_page_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_page_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_paragraph_end_position()` instead."]
    pub fn move_to_previous_paragraph_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_paragraph_end_position(val);
        self
    }
    #[deprecated = "Use `set_as_tree_position()` instead."]
    pub fn as_tree_position(&mut self, val: &Function) -> &mut Self {
        self.set_as_tree_position(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_document()` instead."]
    pub fn at_end_of_document(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_document(val);
        self
    }
    #[deprecated = "Use `set_move_to_position_at_end_of_document()` instead."]
    pub fn move_to_position_at_end_of_document(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_position_at_end_of_document(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_line_start_position()` instead."]
    pub fn move_to_previous_line_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_line_start_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_format_start_position()` instead."]
    pub fn move_to_next_format_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_format_start_position(val);
        self
    }
    #[deprecated = "Use `set_is_null_position()` instead."]
    pub fn is_null_position(&mut self, val: &Function) -> &mut Self {
        self.set_is_null_position(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_line()` instead."]
    pub fn at_start_of_line(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_line(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_format_end_position()` instead."]
    pub fn move_to_previous_format_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_format_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_format_start_position()` instead."]
    pub fn move_to_previous_format_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_format_start_position(val);
        self
    }
    #[deprecated = "Use `set_is_leaf_text_position()` instead."]
    pub fn is_leaf_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_is_leaf_text_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_parent_position()` instead."]
    pub fn move_to_parent_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_parent_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_leaf_text_position()` instead."]
    pub fn move_to_next_leaf_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_leaf_text_position(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_line()` instead."]
    pub fn at_end_of_line(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_line(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_word()` instead."]
    pub fn at_end_of_word(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_word(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_paragraph_end_position()` instead."]
    pub fn move_to_next_paragraph_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_paragraph_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_character_position()` instead."]
    pub fn move_to_next_character_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_character_position(val);
        self
    }
    #[deprecated = "Use `set_is_tree_position()` instead."]
    pub fn is_tree_position(&mut self, val: &Function) -> &mut Self {
        self.set_is_tree_position(val);
        self
    }
    #[deprecated = "Use `set_get_text()` instead."]
    pub fn get_text(&mut self, val: &Function) -> &mut Self {
        self.set_get_text(val);
        self
    }
    #[deprecated = "Use `set_as_leaf_text_position()` instead."]
    pub fn as_leaf_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_as_leaf_text_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_page_start_position()` instead."]
    pub fn move_to_previous_page_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_page_start_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_line_end_position()` instead."]
    pub fn move_to_next_line_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_line_end_position(val);
        self
    }
    #[deprecated = "Use `set_as_text_position()` instead."]
    pub fn as_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_as_text_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_word_start_position()` instead."]
    pub fn move_to_next_word_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_word_start_position(val);
        self
    }
    #[deprecated = "Use `set_is_valid()` instead."]
    pub fn is_valid(&mut self, val: &Function) -> &mut Self {
        self.set_is_valid(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_word_end_position()` instead."]
    pub fn move_to_previous_word_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_word_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_position_at_start_of_document()` instead."]
    pub fn move_to_position_at_start_of_document(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_position_at_start_of_document(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_anchor()` instead."]
    pub fn at_end_of_anchor(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_anchor(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_paragraph()` instead."]
    pub fn at_end_of_paragraph(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_paragraph(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_leaf_tree_position()` instead."]
    pub fn move_to_previous_leaf_tree_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_leaf_tree_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_paragraph_start_position()` instead."]
    pub fn move_to_previous_paragraph_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_paragraph_start_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_format_end_position()` instead."]
    pub fn move_to_next_format_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_format_end_position(val);
        self
    }
    #[deprecated = "Use `set_affinity()` instead."]
    pub fn affinity(&mut self, val: String) -> &mut Self {
        self.set_affinity(val);
        self
    }
    #[deprecated = "Use `set_is_text_position()` instead."]
    pub fn is_text_position(&mut self, val: &Function) -> &mut Self {
        self.set_is_text_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_anchor_position()` instead."]
    pub fn move_to_previous_anchor_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_anchor_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_word_end_position()` instead."]
    pub fn move_to_next_word_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_word_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_page_end_position()` instead."]
    pub fn move_to_next_page_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_page_end_position(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_word_start_position()` instead."]
    pub fn move_to_previous_word_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_word_start_position(val);
        self
    }
    #[deprecated = "Use `set_text_offset()` instead."]
    pub fn text_offset(&mut self, val: i32) -> &mut Self {
        self.set_text_offset(val);
        self
    }
    #[deprecated = "Use `set_node()` instead."]
    pub fn node(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_node(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_paragraph()` instead."]
    pub fn at_start_of_paragraph(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_paragraph(val);
        self
    }
    #[deprecated = "Use `set_is_in_text_object()` instead."]
    pub fn is_in_text_object(&mut self, val: &Function) -> &mut Self {
        self.set_is_in_text_object(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_format()` instead."]
    pub fn at_start_of_format(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_format(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_character_position()` instead."]
    pub fn move_to_previous_character_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_character_position(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_page()` instead."]
    pub fn at_start_of_page(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_page(val);
        self
    }
    #[deprecated = "Use `set_move_to_position_at_start_of_anchor()` instead."]
    pub fn move_to_position_at_start_of_anchor(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_position_at_start_of_anchor(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_anchor_position()` instead."]
    pub fn move_to_next_anchor_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_anchor_position(val);
        self
    }
    #[deprecated = "Use `set_at_end_of_page()` instead."]
    pub fn at_end_of_page(&mut self, val: &Function) -> &mut Self {
        self.set_at_end_of_page(val);
        self
    }
    #[deprecated = "Use `set_move_to_next_page_start_position()` instead."]
    pub fn move_to_next_page_start_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_next_page_start_position(val);
        self
    }
    #[deprecated = "Use `set_is_in_line_break()` instead."]
    pub fn is_in_line_break(&mut self, val: &Function) -> &mut Self {
        self.set_is_in_line_break(val);
        self
    }
    #[deprecated = "Use `set_at_start_of_anchor()` instead."]
    pub fn at_start_of_anchor(&mut self, val: &Function) -> &mut Self {
        self.set_at_start_of_anchor(val);
        self
    }
    #[deprecated = "Use `set_move_to_previous_line_end_position()` instead."]
    pub fn move_to_previous_line_end_position(&mut self, val: &Function) -> &mut Self {
        self.set_move_to_previous_line_end_position(val);
        self
    }
}
impl Default for AutomationPosition {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AutomationNode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AutomationNode;
    ///Get the `unclippedBoundsForRange` field of this object.
    #[wasm_bindgen(method, getter = "unclippedBoundsForRange")]
    pub fn get_unclipped_bounds_for_range(this: &AutomationNode) -> Function;
    ///Change the `unclippedBoundsForRange` field of this object.
    #[wasm_bindgen(method, setter = "unclippedBoundsForRange")]
    pub fn set_unclipped_bounds_for_range(this: &AutomationNode, val: &Function);
    ///Get the `isComboBox` field of this object.
    #[wasm_bindgen(method, getter = "isComboBox")]
    pub fn get_is_combo_box(this: &AutomationNode) -> bool;
    ///Change the `isComboBox` field of this object.
    #[wasm_bindgen(method, setter = "isComboBox")]
    pub fn set_is_combo_box(this: &AutomationNode, val: bool);
    ///Get the `display` field of this object.
    #[wasm_bindgen(method, getter = "display")]
    pub fn get_display(this: &AutomationNode) -> Option<String>;
    ///Change the `display` field of this object.
    #[wasm_bindgen(method, setter = "display")]
    pub fn set_display(this: &AutomationNode, val: String);
    ///Get the `underline` field of this object.
    #[wasm_bindgen(method, getter = "underline")]
    pub fn get_underline(this: &AutomationNode) -> bool;
    ///Change the `underline` field of this object.
    #[wasm_bindgen(method, setter = "underline")]
    pub fn set_underline(this: &AutomationNode, val: bool);
    ///Get the `subscript` field of this object.
    #[wasm_bindgen(method, getter = "subscript")]
    pub fn get_subscript(this: &AutomationNode) -> bool;
    ///Change the `subscript` field of this object.
    #[wasm_bindgen(method, setter = "subscript")]
    pub fn set_subscript(this: &AutomationNode, val: bool);
    ///Get the `tableColumnCount` field of this object.
    #[wasm_bindgen(method, getter = "tableColumnCount")]
    pub fn get_table_column_count(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableColumnCount` field of this object.
    #[wasm_bindgen(method, setter = "tableColumnCount")]
    pub fn set_table_column_count(this: &AutomationNode, val: i32);
    ///Get the `docLoaded` field of this object.
    #[wasm_bindgen(method, getter = "docLoaded")]
    pub fn get_doc_loaded(this: &AutomationNode) -> Option<bool>;
    ///Change the `docLoaded` field of this object.
    #[wasm_bindgen(method, setter = "docLoaded")]
    pub fn set_doc_loaded(this: &AutomationNode, val: bool);
    ///Get the `lineThrough` field of this object.
    #[wasm_bindgen(method, getter = "lineThrough")]
    pub fn get_line_through(this: &AutomationNode) -> bool;
    ///Change the `lineThrough` field of this object.
    #[wasm_bindgen(method, setter = "lineThrough")]
    pub fn set_line_through(this: &AutomationNode, val: bool);
    ///Get the `suspendMedia` field of this object.
    #[wasm_bindgen(method, getter = "suspendMedia")]
    pub fn get_suspend_media(this: &AutomationNode) -> Function;
    ///Change the `suspendMedia` field of this object.
    #[wasm_bindgen(method, setter = "suspendMedia")]
    pub fn set_suspend_media(this: &AutomationNode, val: &Function);
    ///Get the `unclippedLocation` field of this object.
    #[wasm_bindgen(method, getter = "unclippedLocation")]
    pub fn get_unclipped_location(this: &AutomationNode) -> Option<Rect>;
    ///Change the `unclippedLocation` field of this object.
    #[wasm_bindgen(method, setter = "unclippedLocation")]
    pub fn set_unclipped_location(this: &AutomationNode, val: &Rect);
    ///Get the `focusObject` field of this object.
    #[wasm_bindgen(method, getter = "focusObject")]
    pub fn get_focus_object(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `focusObject` field of this object.
    #[wasm_bindgen(method, setter = "focusObject")]
    pub fn set_focus_object(this: &AutomationNode, val: &AutomationNode);
    ///Get the `tableColumnIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableColumnIndex")]
    pub fn get_table_column_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableColumnIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableColumnIndex")]
    pub fn set_table_column_index(this: &AutomationNode, val: i32);
    ///Get the `liveRelevant` field of this object.
    #[wasm_bindgen(method, getter = "liveRelevant")]
    pub fn get_live_relevant(this: &AutomationNode) -> Option<String>;
    ///Change the `liveRelevant` field of this object.
    #[wasm_bindgen(method, setter = "liveRelevant")]
    pub fn set_live_relevant(this: &AutomationNode, val: String);
    ///Get the `innerHtml` field of this object.
    #[wasm_bindgen(method, getter = "innerHtml")]
    pub fn get_inner_html(this: &AutomationNode) -> Option<String>;
    ///Change the `innerHtml` field of this object.
    #[wasm_bindgen(method, setter = "innerHtml")]
    pub fn set_inner_html(this: &AutomationNode, val: String);
    ///Get the `indexInParent` field of this object.
    #[wasm_bindgen(method, getter = "indexInParent")]
    pub fn get_index_in_parent(this: &AutomationNode) -> Option<i32>;
    ///Change the `indexInParent` field of this object.
    #[wasm_bindgen(method, setter = "indexInParent")]
    pub fn set_index_in_parent(this: &AutomationNode, val: i32);
    ///Get the `scrollUp` field of this object.
    #[wasm_bindgen(method, getter = "scrollUp")]
    pub fn get_scroll_up(this: &AutomationNode) -> Function;
    ///Change the `scrollUp` field of this object.
    #[wasm_bindgen(method, setter = "scrollUp")]
    pub fn set_scroll_up(this: &AutomationNode, val: &Function);
    ///Get the `tableRowCount` field of this object.
    #[wasm_bindgen(method, getter = "tableRowCount")]
    pub fn get_table_row_count(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableRowCount` field of this object.
    #[wasm_bindgen(method, setter = "tableRowCount")]
    pub fn set_table_row_count(this: &AutomationNode, val: i32);
    ///Get the `getNextTextMatch` field of this object.
    #[wasm_bindgen(method, getter = "getNextTextMatch")]
    pub fn get_get_next_text_match(this: &AutomationNode) -> Function;
    ///Change the `getNextTextMatch` field of this object.
    #[wasm_bindgen(method, setter = "getNextTextMatch")]
    pub fn set_get_next_text_match(this: &AutomationNode, val: &Function);
    ///Get the `scrollXMax` field of this object.
    #[wasm_bindgen(method, getter = "scrollXMax")]
    pub fn get_scroll_x_max(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollXMax` field of this object.
    #[wasm_bindgen(method, setter = "scrollXMax")]
    pub fn set_scroll_x_max(this: &AutomationNode, val: i32);
    ///Get the `colorValue` field of this object.
    #[wasm_bindgen(method, getter = "colorValue")]
    pub fn get_color_value(this: &AutomationNode) -> Option<i32>;
    ///Change the `colorValue` field of this object.
    #[wasm_bindgen(method, setter = "colorValue")]
    pub fn set_color_value(this: &AutomationNode, val: i32);
    ///Get the `nonInlineTextWordStarts` field of this object.
    #[wasm_bindgen(method, getter = "nonInlineTextWordStarts")]
    pub fn get_non_inline_text_word_starts(this: &AutomationNode) -> Option<Array>;
    ///Change the `nonInlineTextWordStarts` field of this object.
    #[wasm_bindgen(method, setter = "nonInlineTextWordStarts")]
    pub fn set_non_inline_text_word_starts(this: &AutomationNode, val: &Array);
    ///Get the `performCustomAction` field of this object.
    #[wasm_bindgen(method, getter = "performCustomAction")]
    pub fn get_perform_custom_action(this: &AutomationNode) -> Function;
    ///Change the `performCustomAction` field of this object.
    #[wasm_bindgen(method, setter = "performCustomAction")]
    pub fn set_perform_custom_action(this: &AutomationNode, val: &Function);
    ///Get the `scrollBackward` field of this object.
    #[wasm_bindgen(method, getter = "scrollBackward")]
    pub fn get_scroll_backward(this: &AutomationNode) -> Function;
    ///Change the `scrollBackward` field of this object.
    #[wasm_bindgen(method, setter = "scrollBackward")]
    pub fn set_scroll_backward(this: &AutomationNode, val: &Function);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &AutomationNode) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &AutomationNode, val: String);
    ///Get the `imageAnnotation` field of this object.
    #[wasm_bindgen(method, getter = "imageAnnotation")]
    pub fn get_image_annotation(this: &AutomationNode) -> Option<String>;
    ///Change the `imageAnnotation` field of this object.
    #[wasm_bindgen(method, setter = "imageAnnotation")]
    pub fn set_image_annotation(this: &AutomationNode, val: String);
    ///Get the `wordEnds` field of this object.
    #[wasm_bindgen(method, getter = "wordEnds")]
    pub fn get_word_ends(this: &AutomationNode) -> Option<Array>;
    ///Change the `wordEnds` field of this object.
    #[wasm_bindgen(method, setter = "wordEnds")]
    pub fn set_word_ends(this: &AutomationNode, val: &Array);
    ///Get the `detailsFor` field of this object.
    #[wasm_bindgen(method, getter = "detailsFor")]
    pub fn get_details_for(this: &AutomationNode) -> Option<Array>;
    ///Change the `detailsFor` field of this object.
    #[wasm_bindgen(method, setter = "detailsFor")]
    pub fn set_details_for(this: &AutomationNode, val: &Array);
    ///Get the `controls` field of this object.
    #[wasm_bindgen(method, getter = "controls")]
    pub fn get_controls(this: &AutomationNode) -> Option<Array>;
    ///Change the `controls` field of this object.
    #[wasm_bindgen(method, setter = "controls")]
    pub fn set_controls(this: &AutomationNode, val: &Array);
    ///Get the `scrollY` field of this object.
    #[wasm_bindgen(method, getter = "scrollY")]
    pub fn get_scroll_y(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollY` field of this object.
    #[wasm_bindgen(method, setter = "scrollY")]
    pub fn set_scroll_y(this: &AutomationNode, val: i32);
    ///Get the `hierarchicalLevel` field of this object.
    #[wasm_bindgen(method, getter = "hierarchicalLevel")]
    pub fn get_hierarchical_level(this: &AutomationNode) -> Option<i32>;
    ///Change the `hierarchicalLevel` field of this object.
    #[wasm_bindgen(method, setter = "hierarchicalLevel")]
    pub fn set_hierarchical_level(this: &AutomationNode, val: i32);
    ///Get the `details` field of this object.
    #[wasm_bindgen(method, getter = "details")]
    pub fn get_details(this: &AutomationNode) -> Option<Array>;
    ///Change the `details` field of this object.
    #[wasm_bindgen(method, setter = "details")]
    pub fn set_details(this: &AutomationNode, val: &Array);
    ///Get the `flowFrom` field of this object.
    #[wasm_bindgen(method, getter = "flowFrom")]
    pub fn get_flow_from(this: &AutomationNode) -> Option<Array>;
    ///Change the `flowFrom` field of this object.
    #[wasm_bindgen(method, setter = "flowFrom")]
    pub fn set_flow_from(this: &AutomationNode, val: &Array);
    ///Get the `isRootNode` field of this object.
    #[wasm_bindgen(method, getter = "isRootNode")]
    pub fn get_is_root_node(this: &AutomationNode) -> bool;
    ///Change the `isRootNode` field of this object.
    #[wasm_bindgen(method, setter = "isRootNode")]
    pub fn set_is_root_node(this: &AutomationNode, val: bool);
    ///Get the `focusOffset` field of this object.
    #[wasm_bindgen(method, getter = "focusOffset")]
    pub fn get_focus_offset(this: &AutomationNode) -> Option<i32>;
    ///Change the `focusOffset` field of this object.
    #[wasm_bindgen(method, setter = "focusOffset")]
    pub fn set_focus_offset(this: &AutomationNode, val: i32);
    ///Get the `focus` field of this object.
    #[wasm_bindgen(method, getter = "focus")]
    pub fn get_focus(this: &AutomationNode) -> Function;
    ///Change the `focus` field of this object.
    #[wasm_bindgen(method, setter = "focus")]
    pub fn set_focus(this: &AutomationNode, val: &Function);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &AutomationNode) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &AutomationNode, val: String);
    ///Get the `descriptionFor` field of this object.
    #[wasm_bindgen(method, getter = "descriptionFor")]
    pub fn get_description_for(this: &AutomationNode) -> Option<Array>;
    ///Change the `descriptionFor` field of this object.
    #[wasm_bindgen(method, setter = "descriptionFor")]
    pub fn set_description_for(this: &AutomationNode, val: &Array);
    ///Get the `selectionEndAffinity` field of this object.
    #[wasm_bindgen(method, getter = "selectionEndAffinity")]
    pub fn get_selection_end_affinity(this: &AutomationNode) -> Option<String>;
    ///Change the `selectionEndAffinity` field of this object.
    #[wasm_bindgen(method, setter = "selectionEndAffinity")]
    pub fn set_selection_end_affinity(this: &AutomationNode, val: String);
    ///Get the `hitTestWithReply` field of this object.
    #[wasm_bindgen(method, getter = "hitTestWithReply")]
    pub fn get_hit_test_with_reply(this: &AutomationNode) -> Function;
    ///Change the `hitTestWithReply` field of this object.
    #[wasm_bindgen(method, setter = "hitTestWithReply")]
    pub fn set_hit_test_with_reply(this: &AutomationNode, val: &Function);
    ///Get the `modal` field of this object.
    #[wasm_bindgen(method, getter = "modal")]
    pub fn get_modal(this: &AutomationNode) -> Option<bool>;
    ///Change the `modal` field of this object.
    #[wasm_bindgen(method, setter = "modal")]
    pub fn set_modal(this: &AutomationNode, val: bool);
    ///Get the `detectedLanguage` field of this object.
    #[wasm_bindgen(method, getter = "detectedLanguage")]
    pub fn get_detected_language(this: &AutomationNode) -> Option<String>;
    ///Change the `detectedLanguage` field of this object.
    #[wasm_bindgen(method, setter = "detectedLanguage")]
    pub fn set_detected_language(this: &AutomationNode, val: String);
    ///Get the `errorMessages` field of this object.
    #[wasm_bindgen(method, getter = "errorMessages")]
    pub fn get_error_messages(this: &AutomationNode) -> Option<Array>;
    ///Change the `errorMessages` field of this object.
    #[wasm_bindgen(method, setter = "errorMessages")]
    pub fn set_error_messages(this: &AutomationNode, val: &Array);
    ///Get the `children` field of this object.
    #[wasm_bindgen(method, getter = "children")]
    pub fn get_children(this: &AutomationNode) -> Array;
    ///Change the `children` field of this object.
    #[wasm_bindgen(method, setter = "children")]
    pub fn set_children(this: &AutomationNode, val: &Array);
    ///Get the `scrollYMax` field of this object.
    #[wasm_bindgen(method, getter = "scrollYMax")]
    pub fn get_scroll_y_max(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollYMax` field of this object.
    #[wasm_bindgen(method, setter = "scrollYMax")]
    pub fn set_scroll_y_max(this: &AutomationNode, val: i32);
    ///Get the `tableCellRowSpan` field of this object.
    #[wasm_bindgen(method, getter = "tableCellRowSpan")]
    pub fn get_table_cell_row_span(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellRowSpan` field of this object.
    #[wasm_bindgen(method, setter = "tableCellRowSpan")]
    pub fn set_table_cell_row_span(this: &AutomationNode, val: i32);
    ///Get the `location` field of this object.
    #[wasm_bindgen(method, getter = "location")]
    pub fn get_location(this: &AutomationNode) -> Rect;
    ///Change the `location` field of this object.
    #[wasm_bindgen(method, setter = "location")]
    pub fn set_location(this: &AutomationNode, val: &Rect);
    ///Get the `tableCellAriaRowIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableCellAriaRowIndex")]
    pub fn get_table_cell_aria_row_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellAriaRowIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableCellAriaRowIndex")]
    pub fn set_table_cell_aria_row_index(this: &AutomationNode, val: i32);
    ///Get the `restriction` field of this object.
    #[wasm_bindgen(method, getter = "restriction")]
    pub fn get_restriction(this: &AutomationNode) -> Option<String>;
    ///Change the `restriction` field of this object.
    #[wasm_bindgen(method, setter = "restriction")]
    pub fn set_restriction(this: &AutomationNode, val: String);
    ///Get the `docTitle` field of this object.
    #[wasm_bindgen(method, getter = "docTitle")]
    pub fn get_doc_title(this: &AutomationNode) -> Option<String>;
    ///Change the `docTitle` field of this object.
    #[wasm_bindgen(method, setter = "docTitle")]
    pub fn set_doc_title(this: &AutomationNode, val: String);
    ///Get the `scrollLeft` field of this object.
    #[wasm_bindgen(method, getter = "scrollLeft")]
    pub fn get_scroll_left(this: &AutomationNode) -> Function;
    ///Change the `scrollLeft` field of this object.
    #[wasm_bindgen(method, setter = "scrollLeft")]
    pub fn set_scroll_left(this: &AutomationNode, val: &Function);
    ///Get the `checkedStateDescription` field of this object.
    #[wasm_bindgen(method, getter = "checkedStateDescription")]
    pub fn get_checked_state_description(this: &AutomationNode) -> Option<String>;
    ///Change the `checkedStateDescription` field of this object.
    #[wasm_bindgen(method, setter = "checkedStateDescription")]
    pub fn set_checked_state_description(this: &AutomationNode, val: String);
    ///Get the `minValueForRange` field of this object.
    #[wasm_bindgen(method, getter = "minValueForRange")]
    pub fn get_min_value_for_range(this: &AutomationNode) -> Option<f64>;
    ///Change the `minValueForRange` field of this object.
    #[wasm_bindgen(method, setter = "minValueForRange")]
    pub fn set_min_value_for_range(this: &AutomationNode, val: f64);
    ///Get the `errorMessageFor` field of this object.
    #[wasm_bindgen(method, getter = "errorMessageFor")]
    pub fn get_error_message_for(this: &AutomationNode) -> Option<Array>;
    ///Change the `errorMessageFor` field of this object.
    #[wasm_bindgen(method, setter = "errorMessageFor")]
    pub fn set_error_message_for(this: &AutomationNode, val: &Array);
    ///Get the `selectionStartAffinity` field of this object.
    #[wasm_bindgen(method, getter = "selectionStartAffinity")]
    pub fn get_selection_start_affinity(this: &AutomationNode) -> Option<String>;
    ///Change the `selectionStartAffinity` field of this object.
    #[wasm_bindgen(method, setter = "selectionStartAffinity")]
    pub fn set_selection_start_affinity(this: &AutomationNode, val: String);
    ///Get the `clickable` field of this object.
    #[wasm_bindgen(method, getter = "clickable")]
    pub fn get_clickable(this: &AutomationNode) -> bool;
    ///Change the `clickable` field of this object.
    #[wasm_bindgen(method, setter = "clickable")]
    pub fn set_clickable(this: &AutomationNode, val: bool);
    ///Get the `tableCellColumnIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableCellColumnIndex")]
    pub fn get_table_cell_column_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellColumnIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableCellColumnIndex")]
    pub fn set_table_cell_column_index(this: &AutomationNode, val: i32);
    ///Get the `scrollForward` field of this object.
    #[wasm_bindgen(method, getter = "scrollForward")]
    pub fn get_scroll_forward(this: &AutomationNode) -> Function;
    ///Change the `scrollForward` field of this object.
    #[wasm_bindgen(method, setter = "scrollForward")]
    pub fn set_scroll_forward(this: &AutomationNode, val: &Function);
    ///Get the `placeholder` field of this object.
    #[wasm_bindgen(method, getter = "placeholder")]
    pub fn get_placeholder(this: &AutomationNode) -> Option<String>;
    ///Change the `placeholder` field of this object.
    #[wasm_bindgen(method, setter = "placeholder")]
    pub fn set_placeholder(this: &AutomationNode, val: String);
    ///Get the `nextWindowFocus` field of this object.
    #[wasm_bindgen(method, getter = "nextWindowFocus")]
    pub fn get_next_window_focus(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `nextWindowFocus` field of this object.
    #[wasm_bindgen(method, setter = "nextWindowFocus")]
    pub fn set_next_window_focus(this: &AutomationNode, val: &AutomationNode);
    ///Get the `italic` field of this object.
    #[wasm_bindgen(method, getter = "italic")]
    pub fn get_italic(this: &AutomationNode) -> bool;
    ///Change the `italic` field of this object.
    #[wasm_bindgen(method, setter = "italic")]
    pub fn set_italic(this: &AutomationNode, val: bool);
    ///Get the `showContextMenu` field of this object.
    #[wasm_bindgen(method, getter = "showContextMenu")]
    pub fn get_show_context_menu(this: &AutomationNode) -> Function;
    ///Change the `showContextMenu` field of this object.
    #[wasm_bindgen(method, setter = "showContextMenu")]
    pub fn set_show_context_menu(this: &AutomationNode, val: &Function);
    ///Get the `createPosition` field of this object.
    #[wasm_bindgen(method, getter = "createPosition")]
    pub fn get_create_position(this: &AutomationNode) -> Function;
    ///Change the `createPosition` field of this object.
    #[wasm_bindgen(method, setter = "createPosition")]
    pub fn set_create_position(this: &AutomationNode, val: &Function);
    ///Get the `backgroundColor` field of this object.
    #[wasm_bindgen(method, getter = "backgroundColor")]
    pub fn get_background_color(this: &AutomationNode) -> Option<i32>;
    ///Change the `backgroundColor` field of this object.
    #[wasm_bindgen(method, setter = "backgroundColor")]
    pub fn set_background_color(this: &AutomationNode, val: i32);
    ///Get the `nameFrom` field of this object.
    #[wasm_bindgen(method, getter = "nameFrom")]
    pub fn get_name_from(this: &AutomationNode) -> Option<NameFromType>;
    ///Change the `nameFrom` field of this object.
    #[wasm_bindgen(method, setter = "nameFrom")]
    pub fn set_name_from(this: &AutomationNode, val: NameFromType);
    ///Get the `nonAtomicTextFieldRoot` field of this object.
    #[wasm_bindgen(method, getter = "nonAtomicTextFieldRoot")]
    pub fn get_non_atomic_text_field_root(this: &AutomationNode) -> bool;
    ///Change the `nonAtomicTextFieldRoot` field of this object.
    #[wasm_bindgen(method, setter = "nonAtomicTextFieldRoot")]
    pub fn set_non_atomic_text_field_root(this: &AutomationNode, val: bool);
    ///Get the `getImageData` field of this object.
    #[wasm_bindgen(method, getter = "getImageData")]
    pub fn get_get_image_data(this: &AutomationNode) -> Function;
    ///Change the `getImageData` field of this object.
    #[wasm_bindgen(method, setter = "getImageData")]
    pub fn set_get_image_data(this: &AutomationNode, val: &Function);
    ///Get the `markers` field of this object.
    #[wasm_bindgen(method, getter = "markers")]
    pub fn get_markers(this: &AutomationNode) -> Option<Array>;
    ///Change the `markers` field of this object.
    #[wasm_bindgen(method, setter = "markers")]
    pub fn set_markers(this: &AutomationNode, val: &Array);
    ///Get the `appId` field of this object.
    #[wasm_bindgen(method, getter = "appId")]
    pub fn get_app_id(this: &AutomationNode) -> Option<String>;
    ///Change the `appId` field of this object.
    #[wasm_bindgen(method, setter = "appId")]
    pub fn set_app_id(this: &AutomationNode, val: String);
    ///Get the `autoComplete` field of this object.
    #[wasm_bindgen(method, getter = "autoComplete")]
    pub fn get_auto_complete(this: &AutomationNode) -> Option<String>;
    ///Change the `autoComplete` field of this object.
    #[wasm_bindgen(method, setter = "autoComplete")]
    pub fn set_auto_complete(this: &AutomationNode, val: String);
    ///Get the `isCheckBox` field of this object.
    #[wasm_bindgen(method, getter = "isCheckBox")]
    pub fn get_is_check_box(this: &AutomationNode) -> bool;
    ///Change the `isCheckBox` field of this object.
    #[wasm_bindgen(method, setter = "isCheckBox")]
    pub fn set_is_check_box(this: &AutomationNode, val: bool);
    ///Get the `hasPopup` field of this object.
    #[wasm_bindgen(method, getter = "hasPopup")]
    pub fn get_has_popup(this: &AutomationNode) -> Option<HasPopup>;
    ///Change the `hasPopup` field of this object.
    #[wasm_bindgen(method, setter = "hasPopup")]
    pub fn set_has_popup(this: &AutomationNode, val: HasPopup);
    ///Get the `tableColumnHeader` field of this object.
    #[wasm_bindgen(method, getter = "tableColumnHeader")]
    pub fn get_table_column_header(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `tableColumnHeader` field of this object.
    #[wasm_bindgen(method, setter = "tableColumnHeader")]
    pub fn set_table_column_header(this: &AutomationNode, val: &AutomationNode);
    ///Get the `parent` field of this object.
    #[wasm_bindgen(method, getter = "parent")]
    pub fn get_parent(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `parent` field of this object.
    #[wasm_bindgen(method, setter = "parent")]
    pub fn set_parent(this: &AutomationNode, val: &AutomationNode);
    ///Get the `setSequentialFocusNavigationStartingPoint` field of this object.
    #[wasm_bindgen(method, getter = "setSequentialFocusNavigationStartingPoint")]
    pub fn get_set_sequential_focus_navigation_starting_point(this: &AutomationNode) -> Function;
    ///Change the `setSequentialFocusNavigationStartingPoint` field of this object.
    #[wasm_bindgen(method, setter = "setSequentialFocusNavigationStartingPoint")]
    pub fn set_set_sequential_focus_navigation_starting_point(
        this: &AutomationNode,
        val: &Function,
    );
    ///Get the `selected` field of this object.
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &AutomationNode) -> Option<bool>;
    ///Change the `selected` field of this object.
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &AutomationNode, val: bool);
    ///Get the `root` field of this object.
    #[wasm_bindgen(method, getter = "root")]
    pub fn get_root(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `root` field of this object.
    #[wasm_bindgen(method, setter = "root")]
    pub fn set_root(this: &AutomationNode, val: &AutomationNode);
    ///Get the `anchorObject` field of this object.
    #[wasm_bindgen(method, getter = "anchorObject")]
    pub fn get_anchor_object(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `anchorObject` field of this object.
    #[wasm_bindgen(method, setter = "anchorObject")]
    pub fn set_anchor_object(this: &AutomationNode, val: &AutomationNode);
    ///Get the `makeVisible` field of this object.
    #[wasm_bindgen(method, getter = "makeVisible")]
    pub fn get_make_visible(this: &AutomationNode) -> Function;
    ///Change the `makeVisible` field of this object.
    #[wasm_bindgen(method, setter = "makeVisible")]
    pub fn set_make_visible(this: &AutomationNode, val: &Function);
    ///Get the `imageDataUrl` field of this object.
    #[wasm_bindgen(method, getter = "imageDataUrl")]
    pub fn get_image_data_url(this: &AutomationNode) -> Option<String>;
    ///Change the `imageDataUrl` field of this object.
    #[wasm_bindgen(method, setter = "imageDataUrl")]
    pub fn set_image_data_url(this: &AutomationNode, val: String);
    ///Get the `setValue` field of this object.
    #[wasm_bindgen(method, getter = "setValue")]
    pub fn get_set_value(this: &AutomationNode) -> Function;
    ///Change the `setValue` field of this object.
    #[wasm_bindgen(method, setter = "setValue")]
    pub fn set_set_value(this: &AutomationNode, val: &Function);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &AutomationNode) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &AutomationNode, val: String);
    ///Get the `containerLiveBusy` field of this object.
    #[wasm_bindgen(method, getter = "containerLiveBusy")]
    pub fn get_container_live_busy(this: &AutomationNode) -> Option<bool>;
    ///Change the `containerLiveBusy` field of this object.
    #[wasm_bindgen(method, setter = "containerLiveBusy")]
    pub fn set_container_live_busy(this: &AutomationNode, val: bool);
    ///Get the `busy` field of this object.
    #[wasm_bindgen(method, getter = "busy")]
    pub fn get_busy(this: &AutomationNode) -> Option<bool>;
    ///Change the `busy` field of this object.
    #[wasm_bindgen(method, setter = "busy")]
    pub fn set_busy(this: &AutomationNode, val: bool);
    ///Get the `textSelStart` field of this object.
    #[wasm_bindgen(method, getter = "textSelStart")]
    pub fn get_text_sel_start(this: &AutomationNode) -> Option<i32>;
    ///Change the `textSelStart` field of this object.
    #[wasm_bindgen(method, setter = "textSelStart")]
    pub fn set_text_sel_start(this: &AutomationNode, val: i32);
    ///Get the `liveAtomic` field of this object.
    #[wasm_bindgen(method, getter = "liveAtomic")]
    pub fn get_live_atomic(this: &AutomationNode) -> Option<bool>;
    ///Change the `liveAtomic` field of this object.
    #[wasm_bindgen(method, setter = "liveAtomic")]
    pub fn set_live_atomic(this: &AutomationNode, val: bool);
    ///Get the `selectionStartObject` field of this object.
    #[wasm_bindgen(method, getter = "selectionStartObject")]
    pub fn get_selection_start_object(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `selectionStartObject` field of this object.
    #[wasm_bindgen(method, setter = "selectionStartObject")]
    pub fn set_selection_start_object(this: &AutomationNode, val: &AutomationNode);
    ///Get the `doDefaultLabel` field of this object.
    #[wasm_bindgen(method, getter = "doDefaultLabel")]
    pub fn get_do_default_label(this: &AutomationNode) -> Option<String>;
    ///Change the `doDefaultLabel` field of this object.
    #[wasm_bindgen(method, setter = "doDefaultLabel")]
    pub fn set_do_default_label(this: &AutomationNode, val: String);
    ///Get the `scrollXMin` field of this object.
    #[wasm_bindgen(method, getter = "scrollXMin")]
    pub fn get_scroll_x_min(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollXMin` field of this object.
    #[wasm_bindgen(method, setter = "scrollXMin")]
    pub fn set_scroll_x_min(this: &AutomationNode, val: i32);
    ///Get the `anchorOffset` field of this object.
    #[wasm_bindgen(method, getter = "anchorOffset")]
    pub fn get_anchor_offset(this: &AutomationNode) -> Option<i32>;
    ///Change the `anchorOffset` field of this object.
    #[wasm_bindgen(method, setter = "anchorOffset")]
    pub fn set_anchor_offset(this: &AutomationNode, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AutomationNode) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AutomationNode, val: String);
    ///Get the `tableRowIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableRowIndex")]
    pub fn get_table_row_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableRowIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableRowIndex")]
    pub fn set_table_row_index(this: &AutomationNode, val: i32);
    ///Get the `containerLiveRelevant` field of this object.
    #[wasm_bindgen(method, getter = "containerLiveRelevant")]
    pub fn get_container_live_relevant(this: &AutomationNode) -> Option<String>;
    ///Change the `containerLiveRelevant` field of this object.
    #[wasm_bindgen(method, setter = "containerLiveRelevant")]
    pub fn set_container_live_relevant(this: &AutomationNode, val: String);
    ///Get the `previousOnLine` field of this object.
    #[wasm_bindgen(method, getter = "previousOnLine")]
    pub fn get_previous_on_line(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `previousOnLine` field of this object.
    #[wasm_bindgen(method, setter = "previousOnLine")]
    pub fn set_previous_on_line(this: &AutomationNode, val: &AutomationNode);
    ///Get the `selectionEndOffset` field of this object.
    #[wasm_bindgen(method, getter = "selectionEndOffset")]
    pub fn get_selection_end_offset(this: &AutomationNode) -> Option<i32>;
    ///Change the `selectionEndOffset` field of this object.
    #[wasm_bindgen(method, setter = "selectionEndOffset")]
    pub fn set_selection_end_offset(this: &AutomationNode, val: i32);
    ///Get the `setAccessibilityFocus` field of this object.
    #[wasm_bindgen(method, getter = "setAccessibilityFocus")]
    pub fn get_set_accessibility_focus(this: &AutomationNode) -> Function;
    ///Change the `setAccessibilityFocus` field of this object.
    #[wasm_bindgen(method, setter = "setAccessibilityFocus")]
    pub fn set_set_accessibility_focus(this: &AutomationNode, val: &Function);
    ///Get the `previousWindowFocus` field of this object.
    #[wasm_bindgen(method, getter = "previousWindowFocus")]
    pub fn get_previous_window_focus(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `previousWindowFocus` field of this object.
    #[wasm_bindgen(method, setter = "previousWindowFocus")]
    pub fn set_previous_window_focus(this: &AutomationNode, val: &AutomationNode);
    ///Get the `containerLiveAtomic` field of this object.
    #[wasm_bindgen(method, getter = "containerLiveAtomic")]
    pub fn get_container_live_atomic(this: &AutomationNode) -> Option<bool>;
    ///Change the `containerLiveAtomic` field of this object.
    #[wasm_bindgen(method, setter = "containerLiveAtomic")]
    pub fn set_container_live_atomic(this: &AutomationNode, val: bool);
    ///Get the `longClick` field of this object.
    #[wasm_bindgen(method, getter = "longClick")]
    pub fn get_long_click(this: &AutomationNode) -> Function;
    ///Change the `longClick` field of this object.
    #[wasm_bindgen(method, setter = "longClick")]
    pub fn set_long_click(this: &AutomationNode, val: &Function);
    ///Get the `setScrollOffset` field of this object.
    #[wasm_bindgen(method, getter = "setScrollOffset")]
    pub fn get_set_scroll_offset(this: &AutomationNode) -> Function;
    ///Change the `setScrollOffset` field of this object.
    #[wasm_bindgen(method, setter = "setScrollOffset")]
    pub fn set_set_scroll_offset(this: &AutomationNode, val: &Function);
    ///Get the `performStandardAction` field of this object.
    #[wasm_bindgen(method, getter = "performStandardAction")]
    pub fn get_perform_standard_action(this: &AutomationNode) -> Function;
    ///Change the `performStandardAction` field of this object.
    #[wasm_bindgen(method, setter = "performStandardAction")]
    pub fn set_perform_standard_action(this: &AutomationNode, val: &Function);
    ///Get the `doDefault` field of this object.
    #[wasm_bindgen(method, getter = "doDefault")]
    pub fn get_do_default(this: &AutomationNode) -> Function;
    ///Change the `doDefault` field of this object.
    #[wasm_bindgen(method, setter = "doDefault")]
    pub fn set_do_default(this: &AutomationNode, val: &Function);
    ///Get the `resumeMedia` field of this object.
    #[wasm_bindgen(method, getter = "resumeMedia")]
    pub fn get_resume_media(this: &AutomationNode) -> Function;
    ///Change the `resumeMedia` field of this object.
    #[wasm_bindgen(method, setter = "resumeMedia")]
    pub fn set_resume_media(this: &AutomationNode, val: &Function);
    ///Get the `scrollYMin` field of this object.
    #[wasm_bindgen(method, getter = "scrollYMin")]
    pub fn get_scroll_y_min(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollYMin` field of this object.
    #[wasm_bindgen(method, setter = "scrollYMin")]
    pub fn set_scroll_y_min(this: &AutomationNode, val: i32);
    ///Get the `fontFamily` field of this object.
    #[wasm_bindgen(method, getter = "fontFamily")]
    pub fn get_font_family(this: &AutomationNode) -> String;
    ///Change the `fontFamily` field of this object.
    #[wasm_bindgen(method, setter = "fontFamily")]
    pub fn set_font_family(this: &AutomationNode, val: String);
    ///Get the `customActions` field of this object.
    #[wasm_bindgen(method, getter = "customActions")]
    pub fn get_custom_actions(this: &AutomationNode) -> Option<Array>;
    ///Change the `customActions` field of this object.
    #[wasm_bindgen(method, setter = "customActions")]
    pub fn set_custom_actions(this: &AutomationNode, val: &Array);
    ///Get the `sentenceStarts` field of this object.
    #[wasm_bindgen(method, getter = "sentenceStarts")]
    pub fn get_sentence_starts(this: &AutomationNode) -> Option<Array>;
    ///Change the `sentenceStarts` field of this object.
    #[wasm_bindgen(method, setter = "sentenceStarts")]
    pub fn set_sentence_starts(this: &AutomationNode, val: &Array);
    ///Get the `notUserSelectableStyle` field of this object.
    #[wasm_bindgen(method, getter = "notUserSelectableStyle")]
    pub fn get_not_user_selectable_style(this: &AutomationNode) -> Option<bool>;
    ///Change the `notUserSelectableStyle` field of this object.
    #[wasm_bindgen(method, setter = "notUserSelectableStyle")]
    pub fn set_not_user_selectable_style(this: &AutomationNode, val: bool);
    ///Get the `ariaCurrentState` field of this object.
    #[wasm_bindgen(method, getter = "ariaCurrentState")]
    pub fn get_aria_current_state(this: &AutomationNode) -> Option<AriaCurrentState>;
    ///Change the `ariaCurrentState` field of this object.
    #[wasm_bindgen(method, setter = "ariaCurrentState")]
    pub fn set_aria_current_state(this: &AutomationNode, val: AriaCurrentState);
    ///Get the `activeDescendant` field of this object.
    #[wasm_bindgen(method, getter = "activeDescendant")]
    pub fn get_active_descendant(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `activeDescendant` field of this object.
    #[wasm_bindgen(method, setter = "activeDescendant")]
    pub fn set_active_descendant(this: &AutomationNode, val: &AutomationNode);
    ///Get the `firstChild` field of this object.
    #[wasm_bindgen(method, getter = "firstChild")]
    pub fn get_first_child(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `firstChild` field of this object.
    #[wasm_bindgen(method, setter = "firstChild")]
    pub fn set_first_child(this: &AutomationNode, val: &AutomationNode);
    ///Get the `findAll` field of this object.
    #[wasm_bindgen(method, getter = "findAll")]
    pub fn get_find_all(this: &AutomationNode) -> Function;
    ///Change the `findAll` field of this object.
    #[wasm_bindgen(method, setter = "findAll")]
    pub fn set_find_all(this: &AutomationNode, val: &Function);
    ///Get the `maxValueForRange` field of this object.
    #[wasm_bindgen(method, getter = "maxValueForRange")]
    pub fn get_max_value_for_range(this: &AutomationNode) -> Option<f64>;
    ///Change the `maxValueForRange` field of this object.
    #[wasm_bindgen(method, setter = "maxValueForRange")]
    pub fn set_max_value_for_range(this: &AutomationNode, val: f64);
    ///Get the `describedBy` field of this object.
    #[wasm_bindgen(method, getter = "describedBy")]
    pub fn get_described_by(this: &AutomationNode) -> Option<Array>;
    ///Change the `describedBy` field of this object.
    #[wasm_bindgen(method, setter = "describedBy")]
    pub fn set_described_by(this: &AutomationNode, val: &Array);
    ///Get the `docLoadingProgress` field of this object.
    #[wasm_bindgen(method, getter = "docLoadingProgress")]
    pub fn get_doc_loading_progress(this: &AutomationNode) -> Option<f64>;
    ///Change the `docLoadingProgress` field of this object.
    #[wasm_bindgen(method, setter = "docLoadingProgress")]
    pub fn set_doc_loading_progress(this: &AutomationNode, val: f64);
    ///Get the `boundsForRange` field of this object.
    #[wasm_bindgen(method, getter = "boundsForRange")]
    pub fn get_bounds_for_range(this: &AutomationNode) -> Function;
    ///Change the `boundsForRange` field of this object.
    #[wasm_bindgen(method, setter = "boundsForRange")]
    pub fn set_bounds_for_range(this: &AutomationNode, val: &Function);
    ///Get the `superscript` field of this object.
    #[wasm_bindgen(method, getter = "superscript")]
    pub fn get_superscript(this: &AutomationNode) -> bool;
    ///Change the `superscript` field of this object.
    #[wasm_bindgen(method, setter = "superscript")]
    pub fn set_superscript(this: &AutomationNode, val: bool);
    ///Get the `replaceSelectedText` field of this object.
    #[wasm_bindgen(method, getter = "replaceSelectedText")]
    pub fn get_replace_selected_text(this: &AutomationNode) -> Function;
    ///Change the `replaceSelectedText` field of this object.
    #[wasm_bindgen(method, setter = "replaceSelectedText")]
    pub fn set_replace_selected_text(this: &AutomationNode, val: &Function);
    ///Get the `scrollToPoint` field of this object.
    #[wasm_bindgen(method, getter = "scrollToPoint")]
    pub fn get_scroll_to_point(this: &AutomationNode) -> Function;
    ///Change the `scrollToPoint` field of this object.
    #[wasm_bindgen(method, setter = "scrollToPoint")]
    pub fn set_scroll_to_point(this: &AutomationNode, val: &Function);
    ///Get the `isButton` field of this object.
    #[wasm_bindgen(method, getter = "isButton")]
    pub fn get_is_button(this: &AutomationNode) -> bool;
    ///Change the `isButton` field of this object.
    #[wasm_bindgen(method, setter = "isButton")]
    pub fn set_is_button(this: &AutomationNode, val: bool);
    ///Get the `invalidState` field of this object.
    #[wasm_bindgen(method, getter = "invalidState")]
    pub fn get_invalid_state(this: &AutomationNode) -> Option<InvalidState>;
    ///Change the `invalidState` field of this object.
    #[wasm_bindgen(method, setter = "invalidState")]
    pub fn set_invalid_state(this: &AutomationNode, val: InvalidState);
    ///Get the `sentenceEnds` field of this object.
    #[wasm_bindgen(method, getter = "sentenceEnds")]
    pub fn get_sentence_ends(this: &AutomationNode) -> Option<Array>;
    ///Change the `sentenceEnds` field of this object.
    #[wasm_bindgen(method, setter = "sentenceEnds")]
    pub fn set_sentence_ends(this: &AutomationNode, val: &Array);
    ///Get the `containerLiveStatus` field of this object.
    #[wasm_bindgen(method, getter = "containerLiveStatus")]
    pub fn get_container_live_status(this: &AutomationNode) -> Option<String>;
    ///Change the `containerLiveStatus` field of this object.
    #[wasm_bindgen(method, setter = "containerLiveStatus")]
    pub fn set_container_live_status(this: &AutomationNode, val: String);
    ///Get the `htmlId` field of this object.
    #[wasm_bindgen(method, getter = "htmlId")]
    pub fn get_html_id(this: &AutomationNode) -> Option<String>;
    ///Change the `htmlId` field of this object.
    #[wasm_bindgen(method, setter = "htmlId")]
    pub fn set_html_id(this: &AutomationNode, val: String);
    ///Get the `flowTo` field of this object.
    #[wasm_bindgen(method, getter = "flowTo")]
    pub fn get_flow_to(this: &AutomationNode) -> Option<Array>;
    ///Change the `flowTo` field of this object.
    #[wasm_bindgen(method, setter = "flowTo")]
    pub fn set_flow_to(this: &AutomationNode, val: &Array);
    ///Get the `roleDescription` field of this object.
    #[wasm_bindgen(method, getter = "roleDescription")]
    pub fn get_role_description(this: &AutomationNode) -> Option<String>;
    ///Change the `roleDescription` field of this object.
    #[wasm_bindgen(method, setter = "roleDescription")]
    pub fn set_role_description(this: &AutomationNode, val: String);
    ///Get the `labelledBy` field of this object.
    #[wasm_bindgen(method, getter = "labelledBy")]
    pub fn get_labelled_by(this: &AutomationNode) -> Option<Array>;
    ///Change the `labelledBy` field of this object.
    #[wasm_bindgen(method, setter = "labelledBy")]
    pub fn set_labelled_by(this: &AutomationNode, val: &Array);
    ///Get the `tableCellRowHeaders` field of this object.
    #[wasm_bindgen(method, getter = "tableCellRowHeaders")]
    pub fn get_table_cell_row_headers(this: &AutomationNode) -> Option<Array>;
    ///Change the `tableCellRowHeaders` field of this object.
    #[wasm_bindgen(method, setter = "tableCellRowHeaders")]
    pub fn set_table_cell_row_headers(this: &AutomationNode, val: &Array);
    ///Get the `focusAffinity` field of this object.
    #[wasm_bindgen(method, getter = "focusAffinity")]
    pub fn get_focus_affinity(this: &AutomationNode) -> Option<String>;
    ///Change the `focusAffinity` field of this object.
    #[wasm_bindgen(method, setter = "focusAffinity")]
    pub fn set_focus_affinity(this: &AutomationNode, val: String);
    ///Get the `selectionEndObject` field of this object.
    #[wasm_bindgen(method, getter = "selectionEndObject")]
    pub fn get_selection_end_object(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `selectionEndObject` field of this object.
    #[wasm_bindgen(method, setter = "selectionEndObject")]
    pub fn set_selection_end_object(this: &AutomationNode, val: &AutomationNode);
    ///Get the `htmlTag` field of this object.
    #[wasm_bindgen(method, getter = "htmlTag")]
    pub fn get_html_tag(this: &AutomationNode) -> Option<String>;
    ///Change the `htmlTag` field of this object.
    #[wasm_bindgen(method, setter = "htmlTag")]
    pub fn set_html_tag(this: &AutomationNode, val: String);
    ///Get the `scrollX` field of this object.
    #[wasm_bindgen(method, getter = "scrollX")]
    pub fn get_scroll_x(this: &AutomationNode) -> Option<i32>;
    ///Change the `scrollX` field of this object.
    #[wasm_bindgen(method, setter = "scrollX")]
    pub fn set_scroll_x(this: &AutomationNode, val: i32);
    ///Get the `inputType` field of this object.
    #[wasm_bindgen(method, getter = "inputType")]
    pub fn get_input_type(this: &AutomationNode) -> Option<String>;
    ///Change the `inputType` field of this object.
    #[wasm_bindgen(method, setter = "inputType")]
    pub fn set_input_type(this: &AutomationNode, val: String);
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &AutomationNode) -> Option<i32>;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &AutomationNode, val: i32);
    ///Get the `lastChild` field of this object.
    #[wasm_bindgen(method, getter = "lastChild")]
    pub fn get_last_child(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `lastChild` field of this object.
    #[wasm_bindgen(method, setter = "lastChild")]
    pub fn set_last_child(this: &AutomationNode, val: &AutomationNode);
    ///Get the `fontSize` field of this object.
    #[wasm_bindgen(method, getter = "fontSize")]
    pub fn get_font_size(this: &AutomationNode) -> Option<i32>;
    ///Change the `fontSize` field of this object.
    #[wasm_bindgen(method, setter = "fontSize")]
    pub fn set_font_size(this: &AutomationNode, val: i32);
    ///Get the `previousSibling` field of this object.
    #[wasm_bindgen(method, getter = "previousSibling")]
    pub fn get_previous_sibling(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `previousSibling` field of this object.
    #[wasm_bindgen(method, setter = "previousSibling")]
    pub fn set_previous_sibling(this: &AutomationNode, val: &AutomationNode);
    ///Get the `scrollRight` field of this object.
    #[wasm_bindgen(method, getter = "scrollRight")]
    pub fn get_scroll_right(this: &AutomationNode) -> Function;
    ///Change the `scrollRight` field of this object.
    #[wasm_bindgen(method, setter = "scrollRight")]
    pub fn set_scroll_right(this: &AutomationNode, val: &Function);
    ///Get the `docUrl` field of this object.
    #[wasm_bindgen(method, getter = "docUrl")]
    pub fn get_doc_url(this: &AutomationNode) -> Option<String>;
    ///Change the `docUrl` field of this object.
    #[wasm_bindgen(method, setter = "docUrl")]
    pub fn set_doc_url(this: &AutomationNode, val: String);
    ///Get the `liveStatus` field of this object.
    #[wasm_bindgen(method, getter = "liveStatus")]
    pub fn get_live_status(this: &AutomationNode) -> Option<String>;
    ///Change the `liveStatus` field of this object.
    #[wasm_bindgen(method, setter = "liveStatus")]
    pub fn set_live_status(this: &AutomationNode, val: String);
    ///Get the `tableCellColumnHeaders` field of this object.
    #[wasm_bindgen(method, getter = "tableCellColumnHeaders")]
    pub fn get_table_cell_column_headers(this: &AutomationNode) -> Option<Array>;
    ///Change the `tableCellColumnHeaders` field of this object.
    #[wasm_bindgen(method, setter = "tableCellColumnHeaders")]
    pub fn set_table_cell_column_headers(this: &AutomationNode, val: &Array);
    ///Get the `setSize` field of this object.
    #[wasm_bindgen(method, getter = "setSize")]
    pub fn get_set_size(this: &AutomationNode) -> Option<i32>;
    ///Change the `setSize` field of this object.
    #[wasm_bindgen(method, setter = "setSize")]
    pub fn set_set_size(this: &AutomationNode, val: i32);
    ///Get the `bold` field of this object.
    #[wasm_bindgen(method, getter = "bold")]
    pub fn get_bold(this: &AutomationNode) -> bool;
    ///Change the `bold` field of this object.
    #[wasm_bindgen(method, setter = "bold")]
    pub fn set_bold(this: &AutomationNode, val: bool);
    ///Get the `tooltip` field of this object.
    #[wasm_bindgen(method, getter = "tooltip")]
    pub fn get_tooltip(this: &AutomationNode) -> Option<String>;
    ///Change the `tooltip` field of this object.
    #[wasm_bindgen(method, setter = "tooltip")]
    pub fn set_tooltip(this: &AutomationNode, val: String);
    ///Get the `labelFor` field of this object.
    #[wasm_bindgen(method, getter = "labelFor")]
    pub fn get_label_for(this: &AutomationNode) -> Option<Array>;
    ///Change the `labelFor` field of this object.
    #[wasm_bindgen(method, setter = "labelFor")]
    pub fn set_label_for(this: &AutomationNode, val: &Array);
    ///Get the `hasHiddenOffscreenNodes` field of this object.
    #[wasm_bindgen(method, getter = "hasHiddenOffscreenNodes")]
    pub fn get_has_hidden_offscreen_nodes(this: &AutomationNode) -> bool;
    ///Change the `hasHiddenOffscreenNodes` field of this object.
    #[wasm_bindgen(method, setter = "hasHiddenOffscreenNodes")]
    pub fn set_has_hidden_offscreen_nodes(this: &AutomationNode, val: bool);
    ///Get the `scrollable` field of this object.
    #[wasm_bindgen(method, getter = "scrollable")]
    pub fn get_scrollable(this: &AutomationNode) -> Option<bool>;
    ///Change the `scrollable` field of this object.
    #[wasm_bindgen(method, setter = "scrollable")]
    pub fn set_scrollable(this: &AutomationNode, val: bool);
    ///Get the `anchorAffinity` field of this object.
    #[wasm_bindgen(method, getter = "anchorAffinity")]
    pub fn get_anchor_affinity(this: &AutomationNode) -> Option<String>;
    ///Change the `anchorAffinity` field of this object.
    #[wasm_bindgen(method, setter = "anchorAffinity")]
    pub fn set_anchor_affinity(this: &AutomationNode, val: String);
    ///Get the `posInSet` field of this object.
    #[wasm_bindgen(method, getter = "posInSet")]
    pub fn get_pos_in_set(this: &AutomationNode) -> Option<i32>;
    ///Change the `posInSet` field of this object.
    #[wasm_bindgen(method, setter = "posInSet")]
    pub fn set_pos_in_set(this: &AutomationNode, val: i32);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &AutomationNode) -> Option<Object>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &AutomationNode, val: &Object);
    ///Get the `tableRowHeader` field of this object.
    #[wasm_bindgen(method, getter = "tableRowHeader")]
    pub fn get_table_row_header(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `tableRowHeader` field of this object.
    #[wasm_bindgen(method, setter = "tableRowHeader")]
    pub fn set_table_row_header(this: &AutomationNode, val: &AutomationNode);
    ///Get the `ariaInvalidValue` field of this object.
    #[wasm_bindgen(method, getter = "ariaInvalidValue")]
    pub fn get_aria_invalid_value(this: &AutomationNode) -> Option<String>;
    ///Change the `ariaInvalidValue` field of this object.
    #[wasm_bindgen(method, setter = "ariaInvalidValue")]
    pub fn set_aria_invalid_value(this: &AutomationNode, val: String);
    ///Get the `tableCellAriaColumnIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableCellAriaColumnIndex")]
    pub fn get_table_cell_aria_column_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellAriaColumnIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableCellAriaColumnIndex")]
    pub fn set_table_cell_aria_column_index(this: &AutomationNode, val: i32);
    ///Get the `tableCellColumnSpan` field of this object.
    #[wasm_bindgen(method, getter = "tableCellColumnSpan")]
    pub fn get_table_cell_column_span(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellColumnSpan` field of this object.
    #[wasm_bindgen(method, setter = "tableCellColumnSpan")]
    pub fn set_table_cell_column_span(this: &AutomationNode, val: i32);
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &AutomationNode) -> Option<String>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &AutomationNode, val: String);
    ///Get the `controlledBy` field of this object.
    #[wasm_bindgen(method, getter = "controlledBy")]
    pub fn get_controlled_by(this: &AutomationNode) -> Option<Array>;
    ///Change the `controlledBy` field of this object.
    #[wasm_bindgen(method, setter = "controlledBy")]
    pub fn set_controlled_by(this: &AutomationNode, val: &Array);
    ///Get the `nextSibling` field of this object.
    #[wasm_bindgen(method, getter = "nextSibling")]
    pub fn get_next_sibling(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `nextSibling` field of this object.
    #[wasm_bindgen(method, setter = "nextSibling")]
    pub fn set_next_sibling(this: &AutomationNode, val: &AutomationNode);
    ///Get the `nextOnLine` field of this object.
    #[wasm_bindgen(method, getter = "nextOnLine")]
    pub fn get_next_on_line(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `nextOnLine` field of this object.
    #[wasm_bindgen(method, setter = "nextOnLine")]
    pub fn set_next_on_line(this: &AutomationNode, val: &AutomationNode);
    ///Get the `selectionStartOffset` field of this object.
    #[wasm_bindgen(method, getter = "selectionStartOffset")]
    pub fn get_selection_start_offset(this: &AutomationNode) -> Option<i32>;
    ///Change the `selectionStartOffset` field of this object.
    #[wasm_bindgen(method, setter = "selectionStartOffset")]
    pub fn set_selection_start_offset(this: &AutomationNode, val: i32);
    ///Get the `setSelection` field of this object.
    #[wasm_bindgen(method, getter = "setSelection")]
    pub fn get_set_selection(this: &AutomationNode) -> Function;
    ///Change the `setSelection` field of this object.
    #[wasm_bindgen(method, setter = "setSelection")]
    pub fn set_set_selection(this: &AutomationNode, val: &Function);
    ///Get the `addEventListener` field of this object.
    #[wasm_bindgen(method, getter = "addEventListener")]
    pub fn get_add_event_listener(this: &AutomationNode) -> Function;
    ///Change the `addEventListener` field of this object.
    #[wasm_bindgen(method, setter = "addEventListener")]
    pub fn set_add_event_listener(this: &AutomationNode, val: &Function);
    ///Get the `tableCellRowIndex` field of this object.
    #[wasm_bindgen(method, getter = "tableCellRowIndex")]
    pub fn get_table_cell_row_index(this: &AutomationNode) -> Option<i32>;
    ///Change the `tableCellRowIndex` field of this object.
    #[wasm_bindgen(method, setter = "tableCellRowIndex")]
    pub fn set_table_cell_row_index(this: &AutomationNode, val: i32);
    ///Get the `hitTest` field of this object.
    #[wasm_bindgen(method, getter = "hitTest")]
    pub fn get_hit_test(this: &AutomationNode) -> Function;
    ///Change the `hitTest` field of this object.
    #[wasm_bindgen(method, setter = "hitTest")]
    pub fn set_hit_test(this: &AutomationNode, val: &Function);
    ///Get the `wordStarts` field of this object.
    #[wasm_bindgen(method, getter = "wordStarts")]
    pub fn get_word_starts(this: &AutomationNode) -> Option<Array>;
    ///Change the `wordStarts` field of this object.
    #[wasm_bindgen(method, setter = "wordStarts")]
    pub fn set_word_starts(this: &AutomationNode, val: &Array);
    ///Get the `startDuckingMedia` field of this object.
    #[wasm_bindgen(method, getter = "startDuckingMedia")]
    pub fn get_start_ducking_media(this: &AutomationNode) -> Function;
    ///Change the `startDuckingMedia` field of this object.
    #[wasm_bindgen(method, setter = "startDuckingMedia")]
    pub fn set_start_ducking_media(this: &AutomationNode, val: &Function);
    ///Get the `role` field of this object.
    #[wasm_bindgen(method, getter = "role")]
    pub fn get_role(this: &AutomationNode) -> Option<RoleType>;
    ///Change the `role` field of this object.
    #[wasm_bindgen(method, setter = "role")]
    pub fn set_role(this: &AutomationNode, val: RoleType);
    ///Get the `stopDuckingMedia` field of this object.
    #[wasm_bindgen(method, getter = "stopDuckingMedia")]
    pub fn get_stop_ducking_media(this: &AutomationNode) -> Function;
    ///Change the `stopDuckingMedia` field of this object.
    #[wasm_bindgen(method, setter = "stopDuckingMedia")]
    pub fn set_stop_ducking_media(this: &AutomationNode, val: &Function);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &AutomationNode) -> Function;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &AutomationNode, val: &Function);
    ///Get the `textSelEnd` field of this object.
    #[wasm_bindgen(method, getter = "textSelEnd")]
    pub fn get_text_sel_end(this: &AutomationNode) -> Option<i32>;
    ///Change the `textSelEnd` field of this object.
    #[wasm_bindgen(method, setter = "textSelEnd")]
    pub fn set_text_sel_end(this: &AutomationNode, val: i32);
    ///Get the `removeEventListener` field of this object.
    #[wasm_bindgen(method, getter = "removeEventListener")]
    pub fn get_remove_event_listener(this: &AutomationNode) -> Function;
    ///Change the `removeEventListener` field of this object.
    #[wasm_bindgen(method, setter = "removeEventListener")]
    pub fn set_remove_event_listener(this: &AutomationNode, val: &Function);
    ///Get the `nonInlineTextWordEnds` field of this object.
    #[wasm_bindgen(method, getter = "nonInlineTextWordEnds")]
    pub fn get_non_inline_text_word_ends(this: &AutomationNode) -> Option<Array>;
    ///Change the `nonInlineTextWordEnds` field of this object.
    #[wasm_bindgen(method, setter = "nonInlineTextWordEnds")]
    pub fn set_non_inline_text_word_ends(this: &AutomationNode, val: &Array);
    ///Get the `isImage` field of this object.
    #[wasm_bindgen(method, getter = "isImage")]
    pub fn get_is_image(this: &AutomationNode) -> bool;
    ///Change the `isImage` field of this object.
    #[wasm_bindgen(method, setter = "isImage")]
    pub fn set_is_image(this: &AutomationNode, val: bool);
    ///Get the `valueForRange` field of this object.
    #[wasm_bindgen(method, getter = "valueForRange")]
    pub fn get_value_for_range(this: &AutomationNode) -> Option<f64>;
    ///Change the `valueForRange` field of this object.
    #[wasm_bindgen(method, setter = "valueForRange")]
    pub fn set_value_for_range(this: &AutomationNode, val: f64);
    ///Get the `ariaColumnCount` field of this object.
    #[wasm_bindgen(method, getter = "ariaColumnCount")]
    pub fn get_aria_column_count(this: &AutomationNode) -> Option<i32>;
    ///Change the `ariaColumnCount` field of this object.
    #[wasm_bindgen(method, setter = "ariaColumnCount")]
    pub fn set_aria_column_count(this: &AutomationNode, val: i32);
    ///Get the `previousFocus` field of this object.
    #[wasm_bindgen(method, getter = "previousFocus")]
    pub fn get_previous_focus(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `previousFocus` field of this object.
    #[wasm_bindgen(method, setter = "previousFocus")]
    pub fn set_previous_focus(this: &AutomationNode, val: &AutomationNode);
    ///Get the `accessKey` field of this object.
    #[wasm_bindgen(method, getter = "accessKey")]
    pub fn get_access_key(this: &AutomationNode) -> Option<String>;
    ///Change the `accessKey` field of this object.
    #[wasm_bindgen(method, setter = "accessKey")]
    pub fn set_access_key(this: &AutomationNode, val: String);
    ///Get the `defaultActionVerb` field of this object.
    #[wasm_bindgen(method, getter = "defaultActionVerb")]
    pub fn get_default_action_verb(this: &AutomationNode) -> Option<DefaultActionVerb>;
    ///Change the `defaultActionVerb` field of this object.
    #[wasm_bindgen(method, setter = "defaultActionVerb")]
    pub fn set_default_action_verb(this: &AutomationNode, val: DefaultActionVerb);
    ///Get the `find` field of this object.
    #[wasm_bindgen(method, getter = "find")]
    pub fn get_find(this: &AutomationNode) -> Function;
    ///Change the `find` field of this object.
    #[wasm_bindgen(method, setter = "find")]
    pub fn set_find(this: &AutomationNode, val: &Function);
    ///Get the `scrollDown` field of this object.
    #[wasm_bindgen(method, getter = "scrollDown")]
    pub fn get_scroll_down(this: &AutomationNode) -> Function;
    ///Change the `scrollDown` field of this object.
    #[wasm_bindgen(method, setter = "scrollDown")]
    pub fn set_scroll_down(this: &AutomationNode, val: &Function);
    ///Get the `longClickLabel` field of this object.
    #[wasm_bindgen(method, getter = "longClickLabel")]
    pub fn get_long_click_label(this: &AutomationNode) -> Option<String>;
    ///Change the `longClickLabel` field of this object.
    #[wasm_bindgen(method, setter = "longClickLabel")]
    pub fn set_long_click_label(this: &AutomationNode, val: String);
    ///Get the `nextFocus` field of this object.
    #[wasm_bindgen(method, getter = "nextFocus")]
    pub fn get_next_focus(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `nextFocus` field of this object.
    #[wasm_bindgen(method, setter = "nextFocus")]
    pub fn set_next_focus(this: &AutomationNode, val: &AutomationNode);
    ///Get the `className` field of this object.
    #[wasm_bindgen(method, getter = "className")]
    pub fn get_class_name(this: &AutomationNode) -> Option<String>;
    ///Change the `className` field of this object.
    #[wasm_bindgen(method, setter = "className")]
    pub fn set_class_name(this: &AutomationNode, val: String);
    ///Get the `sortDirection` field of this object.
    #[wasm_bindgen(method, getter = "sortDirection")]
    pub fn get_sort_direction(this: &AutomationNode) -> SortDirectionType;
    ///Change the `sortDirection` field of this object.
    #[wasm_bindgen(method, setter = "sortDirection")]
    pub fn set_sort_direction(this: &AutomationNode, val: SortDirectionType);
    ///Get the `isSelectionBackward` field of this object.
    #[wasm_bindgen(method, getter = "isSelectionBackward")]
    pub fn get_is_selection_backward(this: &AutomationNode) -> Option<bool>;
    ///Change the `isSelectionBackward` field of this object.
    #[wasm_bindgen(method, setter = "isSelectionBackward")]
    pub fn set_is_selection_backward(this: &AutomationNode, val: bool);
    ///Get the `standardActions` field of this object.
    #[wasm_bindgen(method, getter = "standardActions")]
    pub fn get_standard_actions(this: &AutomationNode) -> Option<Array>;
    ///Change the `standardActions` field of this object.
    #[wasm_bindgen(method, setter = "standardActions")]
    pub fn set_standard_actions(this: &AutomationNode, val: &Array);
    ///Get the `inPageLinkTarget` field of this object.
    #[wasm_bindgen(method, getter = "inPageLinkTarget")]
    pub fn get_in_page_link_target(this: &AutomationNode) -> Option<AutomationNode>;
    ///Change the `inPageLinkTarget` field of this object.
    #[wasm_bindgen(method, setter = "inPageLinkTarget")]
    pub fn set_in_page_link_target(this: &AutomationNode, val: &AutomationNode);
    ///Get the `activeDescendantFor` field of this object.
    #[wasm_bindgen(method, getter = "activeDescendantFor")]
    pub fn get_active_descendant_for(this: &AutomationNode) -> Option<Array>;
    ///Change the `activeDescendantFor` field of this object.
    #[wasm_bindgen(method, setter = "activeDescendantFor")]
    pub fn set_active_descendant_for(this: &AutomationNode, val: &Array);
    ///Get the `language` field of this object.
    #[wasm_bindgen(method, getter = "language")]
    pub fn get_language(this: &AutomationNode) -> Option<String>;
    ///Change the `language` field of this object.
    #[wasm_bindgen(method, setter = "language")]
    pub fn set_language(this: &AutomationNode, val: String);
    ///Get the `caretBounds` field of this object.
    #[wasm_bindgen(method, getter = "caretBounds")]
    pub fn get_caret_bounds(this: &AutomationNode) -> Option<Rect>;
    ///Change the `caretBounds` field of this object.
    #[wasm_bindgen(method, setter = "caretBounds")]
    pub fn set_caret_bounds(this: &AutomationNode, val: &Rect);
    ///Get the `ariaRowCount` field of this object.
    #[wasm_bindgen(method, getter = "ariaRowCount")]
    pub fn get_aria_row_count(this: &AutomationNode) -> Option<i32>;
    ///Change the `ariaRowCount` field of this object.
    #[wasm_bindgen(method, setter = "ariaRowCount")]
    pub fn set_aria_row_count(this: &AutomationNode, val: i32);
}
impl AutomationNode {
    ///Construct a new `AutomationNode`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_unclipped_bounds_for_range()` instead."]
    pub fn unclipped_bounds_for_range(&mut self, val: &Function) -> &mut Self {
        self.set_unclipped_bounds_for_range(val);
        self
    }
    #[deprecated = "Use `set_is_combo_box()` instead."]
    pub fn is_combo_box(&mut self, val: bool) -> &mut Self {
        self.set_is_combo_box(val);
        self
    }
    #[deprecated = "Use `set_display()` instead."]
    pub fn display(&mut self, val: String) -> &mut Self {
        self.set_display(val);
        self
    }
    #[deprecated = "Use `set_underline()` instead."]
    pub fn underline(&mut self, val: bool) -> &mut Self {
        self.set_underline(val);
        self
    }
    #[deprecated = "Use `set_subscript()` instead."]
    pub fn subscript(&mut self, val: bool) -> &mut Self {
        self.set_subscript(val);
        self
    }
    #[deprecated = "Use `set_table_column_count()` instead."]
    pub fn table_column_count(&mut self, val: i32) -> &mut Self {
        self.set_table_column_count(val);
        self
    }
    #[deprecated = "Use `set_doc_loaded()` instead."]
    pub fn doc_loaded(&mut self, val: bool) -> &mut Self {
        self.set_doc_loaded(val);
        self
    }
    #[deprecated = "Use `set_line_through()` instead."]
    pub fn line_through(&mut self, val: bool) -> &mut Self {
        self.set_line_through(val);
        self
    }
    #[deprecated = "Use `set_suspend_media()` instead."]
    pub fn suspend_media(&mut self, val: &Function) -> &mut Self {
        self.set_suspend_media(val);
        self
    }
    #[deprecated = "Use `set_unclipped_location()` instead."]
    pub fn unclipped_location(&mut self, val: &Rect) -> &mut Self {
        self.set_unclipped_location(val);
        self
    }
    #[deprecated = "Use `set_focus_object()` instead."]
    pub fn focus_object(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_focus_object(val);
        self
    }
    #[deprecated = "Use `set_table_column_index()` instead."]
    pub fn table_column_index(&mut self, val: i32) -> &mut Self {
        self.set_table_column_index(val);
        self
    }
    #[deprecated = "Use `set_live_relevant()` instead."]
    pub fn live_relevant(&mut self, val: String) -> &mut Self {
        self.set_live_relevant(val);
        self
    }
    #[deprecated = "Use `set_inner_html()` instead."]
    pub fn inner_html(&mut self, val: String) -> &mut Self {
        self.set_inner_html(val);
        self
    }
    #[deprecated = "Use `set_index_in_parent()` instead."]
    pub fn index_in_parent(&mut self, val: i32) -> &mut Self {
        self.set_index_in_parent(val);
        self
    }
    #[deprecated = "Use `set_scroll_up()` instead."]
    pub fn scroll_up(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_up(val);
        self
    }
    #[deprecated = "Use `set_table_row_count()` instead."]
    pub fn table_row_count(&mut self, val: i32) -> &mut Self {
        self.set_table_row_count(val);
        self
    }
    #[deprecated = "Use `set_get_next_text_match()` instead."]
    pub fn get_next_text_match(&mut self, val: &Function) -> &mut Self {
        self.set_get_next_text_match(val);
        self
    }
    #[deprecated = "Use `set_scroll_x_max()` instead."]
    pub fn scroll_x_max(&mut self, val: i32) -> &mut Self {
        self.set_scroll_x_max(val);
        self
    }
    #[deprecated = "Use `set_color_value()` instead."]
    pub fn color_value(&mut self, val: i32) -> &mut Self {
        self.set_color_value(val);
        self
    }
    #[deprecated = "Use `set_non_inline_text_word_starts()` instead."]
    pub fn non_inline_text_word_starts(&mut self, val: &Array) -> &mut Self {
        self.set_non_inline_text_word_starts(val);
        self
    }
    #[deprecated = "Use `set_perform_custom_action()` instead."]
    pub fn perform_custom_action(&mut self, val: &Function) -> &mut Self {
        self.set_perform_custom_action(val);
        self
    }
    #[deprecated = "Use `set_scroll_backward()` instead."]
    pub fn scroll_backward(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_backward(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_image_annotation()` instead."]
    pub fn image_annotation(&mut self, val: String) -> &mut Self {
        self.set_image_annotation(val);
        self
    }
    #[deprecated = "Use `set_word_ends()` instead."]
    pub fn word_ends(&mut self, val: &Array) -> &mut Self {
        self.set_word_ends(val);
        self
    }
    #[deprecated = "Use `set_details_for()` instead."]
    pub fn details_for(&mut self, val: &Array) -> &mut Self {
        self.set_details_for(val);
        self
    }
    #[deprecated = "Use `set_controls()` instead."]
    pub fn controls(&mut self, val: &Array) -> &mut Self {
        self.set_controls(val);
        self
    }
    #[deprecated = "Use `set_scroll_y()` instead."]
    pub fn scroll_y(&mut self, val: i32) -> &mut Self {
        self.set_scroll_y(val);
        self
    }
    #[deprecated = "Use `set_hierarchical_level()` instead."]
    pub fn hierarchical_level(&mut self, val: i32) -> &mut Self {
        self.set_hierarchical_level(val);
        self
    }
    #[deprecated = "Use `set_details()` instead."]
    pub fn details(&mut self, val: &Array) -> &mut Self {
        self.set_details(val);
        self
    }
    #[deprecated = "Use `set_flow_from()` instead."]
    pub fn flow_from(&mut self, val: &Array) -> &mut Self {
        self.set_flow_from(val);
        self
    }
    #[deprecated = "Use `set_is_root_node()` instead."]
    pub fn is_root_node(&mut self, val: bool) -> &mut Self {
        self.set_is_root_node(val);
        self
    }
    #[deprecated = "Use `set_focus_offset()` instead."]
    pub fn focus_offset(&mut self, val: i32) -> &mut Self {
        self.set_focus_offset(val);
        self
    }
    #[deprecated = "Use `set_focus()` instead."]
    pub fn focus(&mut self, val: &Function) -> &mut Self {
        self.set_focus(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_description_for()` instead."]
    pub fn description_for(&mut self, val: &Array) -> &mut Self {
        self.set_description_for(val);
        self
    }
    #[deprecated = "Use `set_selection_end_affinity()` instead."]
    pub fn selection_end_affinity(&mut self, val: String) -> &mut Self {
        self.set_selection_end_affinity(val);
        self
    }
    #[deprecated = "Use `set_hit_test_with_reply()` instead."]
    pub fn hit_test_with_reply(&mut self, val: &Function) -> &mut Self {
        self.set_hit_test_with_reply(val);
        self
    }
    #[deprecated = "Use `set_modal()` instead."]
    pub fn modal(&mut self, val: bool) -> &mut Self {
        self.set_modal(val);
        self
    }
    #[deprecated = "Use `set_detected_language()` instead."]
    pub fn detected_language(&mut self, val: String) -> &mut Self {
        self.set_detected_language(val);
        self
    }
    #[deprecated = "Use `set_error_messages()` instead."]
    pub fn error_messages(&mut self, val: &Array) -> &mut Self {
        self.set_error_messages(val);
        self
    }
    #[deprecated = "Use `set_children()` instead."]
    pub fn children(&mut self, val: &Array) -> &mut Self {
        self.set_children(val);
        self
    }
    #[deprecated = "Use `set_scroll_y_max()` instead."]
    pub fn scroll_y_max(&mut self, val: i32) -> &mut Self {
        self.set_scroll_y_max(val);
        self
    }
    #[deprecated = "Use `set_table_cell_row_span()` instead."]
    pub fn table_cell_row_span(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_row_span(val);
        self
    }
    #[deprecated = "Use `set_location()` instead."]
    pub fn location(&mut self, val: &Rect) -> &mut Self {
        self.set_location(val);
        self
    }
    #[deprecated = "Use `set_table_cell_aria_row_index()` instead."]
    pub fn table_cell_aria_row_index(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_aria_row_index(val);
        self
    }
    #[deprecated = "Use `set_restriction()` instead."]
    pub fn restriction(&mut self, val: String) -> &mut Self {
        self.set_restriction(val);
        self
    }
    #[deprecated = "Use `set_doc_title()` instead."]
    pub fn doc_title(&mut self, val: String) -> &mut Self {
        self.set_doc_title(val);
        self
    }
    #[deprecated = "Use `set_scroll_left()` instead."]
    pub fn scroll_left(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_left(val);
        self
    }
    #[deprecated = "Use `set_checked_state_description()` instead."]
    pub fn checked_state_description(&mut self, val: String) -> &mut Self {
        self.set_checked_state_description(val);
        self
    }
    #[deprecated = "Use `set_min_value_for_range()` instead."]
    pub fn min_value_for_range(&mut self, val: f64) -> &mut Self {
        self.set_min_value_for_range(val);
        self
    }
    #[deprecated = "Use `set_error_message_for()` instead."]
    pub fn error_message_for(&mut self, val: &Array) -> &mut Self {
        self.set_error_message_for(val);
        self
    }
    #[deprecated = "Use `set_selection_start_affinity()` instead."]
    pub fn selection_start_affinity(&mut self, val: String) -> &mut Self {
        self.set_selection_start_affinity(val);
        self
    }
    #[deprecated = "Use `set_clickable()` instead."]
    pub fn clickable(&mut self, val: bool) -> &mut Self {
        self.set_clickable(val);
        self
    }
    #[deprecated = "Use `set_table_cell_column_index()` instead."]
    pub fn table_cell_column_index(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_column_index(val);
        self
    }
    #[deprecated = "Use `set_scroll_forward()` instead."]
    pub fn scroll_forward(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_forward(val);
        self
    }
    #[deprecated = "Use `set_placeholder()` instead."]
    pub fn placeholder(&mut self, val: String) -> &mut Self {
        self.set_placeholder(val);
        self
    }
    #[deprecated = "Use `set_next_window_focus()` instead."]
    pub fn next_window_focus(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_next_window_focus(val);
        self
    }
    #[deprecated = "Use `set_italic()` instead."]
    pub fn italic(&mut self, val: bool) -> &mut Self {
        self.set_italic(val);
        self
    }
    #[deprecated = "Use `set_show_context_menu()` instead."]
    pub fn show_context_menu(&mut self, val: &Function) -> &mut Self {
        self.set_show_context_menu(val);
        self
    }
    #[deprecated = "Use `set_create_position()` instead."]
    pub fn create_position(&mut self, val: &Function) -> &mut Self {
        self.set_create_position(val);
        self
    }
    #[deprecated = "Use `set_background_color()` instead."]
    pub fn background_color(&mut self, val: i32) -> &mut Self {
        self.set_background_color(val);
        self
    }
    #[deprecated = "Use `set_name_from()` instead."]
    pub fn name_from(&mut self, val: NameFromType) -> &mut Self {
        self.set_name_from(val);
        self
    }
    #[deprecated = "Use `set_non_atomic_text_field_root()` instead."]
    pub fn non_atomic_text_field_root(&mut self, val: bool) -> &mut Self {
        self.set_non_atomic_text_field_root(val);
        self
    }
    #[deprecated = "Use `set_get_image_data()` instead."]
    pub fn get_image_data(&mut self, val: &Function) -> &mut Self {
        self.set_get_image_data(val);
        self
    }
    #[deprecated = "Use `set_markers()` instead."]
    pub fn markers(&mut self, val: &Array) -> &mut Self {
        self.set_markers(val);
        self
    }
    #[deprecated = "Use `set_app_id()` instead."]
    pub fn app_id(&mut self, val: String) -> &mut Self {
        self.set_app_id(val);
        self
    }
    #[deprecated = "Use `set_auto_complete()` instead."]
    pub fn auto_complete(&mut self, val: String) -> &mut Self {
        self.set_auto_complete(val);
        self
    }
    #[deprecated = "Use `set_is_check_box()` instead."]
    pub fn is_check_box(&mut self, val: bool) -> &mut Self {
        self.set_is_check_box(val);
        self
    }
    #[deprecated = "Use `set_has_popup()` instead."]
    pub fn has_popup(&mut self, val: HasPopup) -> &mut Self {
        self.set_has_popup(val);
        self
    }
    #[deprecated = "Use `set_table_column_header()` instead."]
    pub fn table_column_header(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_table_column_header(val);
        self
    }
    #[deprecated = "Use `set_parent()` instead."]
    pub fn parent(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_parent(val);
        self
    }
    #[deprecated = "Use `set_set_sequential_focus_navigation_starting_point()` instead."]
    pub fn set_sequential_focus_navigation_starting_point(&mut self, val: &Function) -> &mut Self {
        self.set_set_sequential_focus_navigation_starting_point(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[deprecated = "Use `set_root()` instead."]
    pub fn root(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_root(val);
        self
    }
    #[deprecated = "Use `set_anchor_object()` instead."]
    pub fn anchor_object(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_anchor_object(val);
        self
    }
    #[deprecated = "Use `set_make_visible()` instead."]
    pub fn make_visible(&mut self, val: &Function) -> &mut Self {
        self.set_make_visible(val);
        self
    }
    #[deprecated = "Use `set_image_data_url()` instead."]
    pub fn image_data_url(&mut self, val: String) -> &mut Self {
        self.set_image_data_url(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
    #[deprecated = "Use `set_container_live_busy()` instead."]
    pub fn container_live_busy(&mut self, val: bool) -> &mut Self {
        self.set_container_live_busy(val);
        self
    }
    #[deprecated = "Use `set_busy()` instead."]
    pub fn busy(&mut self, val: bool) -> &mut Self {
        self.set_busy(val);
        self
    }
    #[deprecated = "Use `set_text_sel_start()` instead."]
    pub fn text_sel_start(&mut self, val: i32) -> &mut Self {
        self.set_text_sel_start(val);
        self
    }
    #[deprecated = "Use `set_live_atomic()` instead."]
    pub fn live_atomic(&mut self, val: bool) -> &mut Self {
        self.set_live_atomic(val);
        self
    }
    #[deprecated = "Use `set_selection_start_object()` instead."]
    pub fn selection_start_object(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_selection_start_object(val);
        self
    }
    #[deprecated = "Use `set_do_default_label()` instead."]
    pub fn do_default_label(&mut self, val: String) -> &mut Self {
        self.set_do_default_label(val);
        self
    }
    #[deprecated = "Use `set_scroll_x_min()` instead."]
    pub fn scroll_x_min(&mut self, val: i32) -> &mut Self {
        self.set_scroll_x_min(val);
        self
    }
    #[deprecated = "Use `set_anchor_offset()` instead."]
    pub fn anchor_offset(&mut self, val: i32) -> &mut Self {
        self.set_anchor_offset(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_table_row_index()` instead."]
    pub fn table_row_index(&mut self, val: i32) -> &mut Self {
        self.set_table_row_index(val);
        self
    }
    #[deprecated = "Use `set_container_live_relevant()` instead."]
    pub fn container_live_relevant(&mut self, val: String) -> &mut Self {
        self.set_container_live_relevant(val);
        self
    }
    #[deprecated = "Use `set_previous_on_line()` instead."]
    pub fn previous_on_line(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_previous_on_line(val);
        self
    }
    #[deprecated = "Use `set_selection_end_offset()` instead."]
    pub fn selection_end_offset(&mut self, val: i32) -> &mut Self {
        self.set_selection_end_offset(val);
        self
    }
    #[deprecated = "Use `set_set_accessibility_focus()` instead."]
    pub fn set_accessibility_focus(&mut self, val: &Function) -> &mut Self {
        self.set_set_accessibility_focus(val);
        self
    }
    #[deprecated = "Use `set_previous_window_focus()` instead."]
    pub fn previous_window_focus(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_previous_window_focus(val);
        self
    }
    #[deprecated = "Use `set_container_live_atomic()` instead."]
    pub fn container_live_atomic(&mut self, val: bool) -> &mut Self {
        self.set_container_live_atomic(val);
        self
    }
    #[deprecated = "Use `set_long_click()` instead."]
    pub fn long_click(&mut self, val: &Function) -> &mut Self {
        self.set_long_click(val);
        self
    }
    #[deprecated = "Use `set_set_scroll_offset()` instead."]
    pub fn set_scroll_offset(&mut self, val: &Function) -> &mut Self {
        self.set_set_scroll_offset(val);
        self
    }
    #[deprecated = "Use `set_perform_standard_action()` instead."]
    pub fn perform_standard_action(&mut self, val: &Function) -> &mut Self {
        self.set_perform_standard_action(val);
        self
    }
    #[deprecated = "Use `set_do_default()` instead."]
    pub fn do_default(&mut self, val: &Function) -> &mut Self {
        self.set_do_default(val);
        self
    }
    #[deprecated = "Use `set_resume_media()` instead."]
    pub fn resume_media(&mut self, val: &Function) -> &mut Self {
        self.set_resume_media(val);
        self
    }
    #[deprecated = "Use `set_scroll_y_min()` instead."]
    pub fn scroll_y_min(&mut self, val: i32) -> &mut Self {
        self.set_scroll_y_min(val);
        self
    }
    #[deprecated = "Use `set_font_family()` instead."]
    pub fn font_family(&mut self, val: String) -> &mut Self {
        self.set_font_family(val);
        self
    }
    #[deprecated = "Use `set_custom_actions()` instead."]
    pub fn custom_actions(&mut self, val: &Array) -> &mut Self {
        self.set_custom_actions(val);
        self
    }
    #[deprecated = "Use `set_sentence_starts()` instead."]
    pub fn sentence_starts(&mut self, val: &Array) -> &mut Self {
        self.set_sentence_starts(val);
        self
    }
    #[deprecated = "Use `set_not_user_selectable_style()` instead."]
    pub fn not_user_selectable_style(&mut self, val: bool) -> &mut Self {
        self.set_not_user_selectable_style(val);
        self
    }
    #[deprecated = "Use `set_aria_current_state()` instead."]
    pub fn aria_current_state(&mut self, val: AriaCurrentState) -> &mut Self {
        self.set_aria_current_state(val);
        self
    }
    #[deprecated = "Use `set_active_descendant()` instead."]
    pub fn active_descendant(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_active_descendant(val);
        self
    }
    #[deprecated = "Use `set_first_child()` instead."]
    pub fn first_child(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_first_child(val);
        self
    }
    #[deprecated = "Use `set_find_all()` instead."]
    pub fn find_all(&mut self, val: &Function) -> &mut Self {
        self.set_find_all(val);
        self
    }
    #[deprecated = "Use `set_max_value_for_range()` instead."]
    pub fn max_value_for_range(&mut self, val: f64) -> &mut Self {
        self.set_max_value_for_range(val);
        self
    }
    #[deprecated = "Use `set_described_by()` instead."]
    pub fn described_by(&mut self, val: &Array) -> &mut Self {
        self.set_described_by(val);
        self
    }
    #[deprecated = "Use `set_doc_loading_progress()` instead."]
    pub fn doc_loading_progress(&mut self, val: f64) -> &mut Self {
        self.set_doc_loading_progress(val);
        self
    }
    #[deprecated = "Use `set_bounds_for_range()` instead."]
    pub fn bounds_for_range(&mut self, val: &Function) -> &mut Self {
        self.set_bounds_for_range(val);
        self
    }
    #[deprecated = "Use `set_superscript()` instead."]
    pub fn superscript(&mut self, val: bool) -> &mut Self {
        self.set_superscript(val);
        self
    }
    #[deprecated = "Use `set_replace_selected_text()` instead."]
    pub fn replace_selected_text(&mut self, val: &Function) -> &mut Self {
        self.set_replace_selected_text(val);
        self
    }
    #[deprecated = "Use `set_scroll_to_point()` instead."]
    pub fn scroll_to_point(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_to_point(val);
        self
    }
    #[deprecated = "Use `set_is_button()` instead."]
    pub fn is_button(&mut self, val: bool) -> &mut Self {
        self.set_is_button(val);
        self
    }
    #[deprecated = "Use `set_invalid_state()` instead."]
    pub fn invalid_state(&mut self, val: InvalidState) -> &mut Self {
        self.set_invalid_state(val);
        self
    }
    #[deprecated = "Use `set_sentence_ends()` instead."]
    pub fn sentence_ends(&mut self, val: &Array) -> &mut Self {
        self.set_sentence_ends(val);
        self
    }
    #[deprecated = "Use `set_container_live_status()` instead."]
    pub fn container_live_status(&mut self, val: String) -> &mut Self {
        self.set_container_live_status(val);
        self
    }
    #[deprecated = "Use `set_html_id()` instead."]
    pub fn html_id(&mut self, val: String) -> &mut Self {
        self.set_html_id(val);
        self
    }
    #[deprecated = "Use `set_flow_to()` instead."]
    pub fn flow_to(&mut self, val: &Array) -> &mut Self {
        self.set_flow_to(val);
        self
    }
    #[deprecated = "Use `set_role_description()` instead."]
    pub fn role_description(&mut self, val: String) -> &mut Self {
        self.set_role_description(val);
        self
    }
    #[deprecated = "Use `set_labelled_by()` instead."]
    pub fn labelled_by(&mut self, val: &Array) -> &mut Self {
        self.set_labelled_by(val);
        self
    }
    #[deprecated = "Use `set_table_cell_row_headers()` instead."]
    pub fn table_cell_row_headers(&mut self, val: &Array) -> &mut Self {
        self.set_table_cell_row_headers(val);
        self
    }
    #[deprecated = "Use `set_focus_affinity()` instead."]
    pub fn focus_affinity(&mut self, val: String) -> &mut Self {
        self.set_focus_affinity(val);
        self
    }
    #[deprecated = "Use `set_selection_end_object()` instead."]
    pub fn selection_end_object(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_selection_end_object(val);
        self
    }
    #[deprecated = "Use `set_html_tag()` instead."]
    pub fn html_tag(&mut self, val: String) -> &mut Self {
        self.set_html_tag(val);
        self
    }
    #[deprecated = "Use `set_scroll_x()` instead."]
    pub fn scroll_x(&mut self, val: i32) -> &mut Self {
        self.set_scroll_x(val);
        self
    }
    #[deprecated = "Use `set_input_type()` instead."]
    pub fn input_type(&mut self, val: String) -> &mut Self {
        self.set_input_type(val);
        self
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: i32) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_last_child()` instead."]
    pub fn last_child(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_last_child(val);
        self
    }
    #[deprecated = "Use `set_font_size()` instead."]
    pub fn font_size(&mut self, val: i32) -> &mut Self {
        self.set_font_size(val);
        self
    }
    #[deprecated = "Use `set_previous_sibling()` instead."]
    pub fn previous_sibling(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_previous_sibling(val);
        self
    }
    #[deprecated = "Use `set_scroll_right()` instead."]
    pub fn scroll_right(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_right(val);
        self
    }
    #[deprecated = "Use `set_doc_url()` instead."]
    pub fn doc_url(&mut self, val: String) -> &mut Self {
        self.set_doc_url(val);
        self
    }
    #[deprecated = "Use `set_live_status()` instead."]
    pub fn live_status(&mut self, val: String) -> &mut Self {
        self.set_live_status(val);
        self
    }
    #[deprecated = "Use `set_table_cell_column_headers()` instead."]
    pub fn table_cell_column_headers(&mut self, val: &Array) -> &mut Self {
        self.set_table_cell_column_headers(val);
        self
    }
    #[deprecated = "Use `set_set_size()` instead."]
    pub fn set_size(&mut self, val: i32) -> &mut Self {
        self.set_set_size(val);
        self
    }
    #[deprecated = "Use `set_bold()` instead."]
    pub fn bold(&mut self, val: bool) -> &mut Self {
        self.set_bold(val);
        self
    }
    #[deprecated = "Use `set_tooltip()` instead."]
    pub fn tooltip(&mut self, val: String) -> &mut Self {
        self.set_tooltip(val);
        self
    }
    #[deprecated = "Use `set_label_for()` instead."]
    pub fn label_for(&mut self, val: &Array) -> &mut Self {
        self.set_label_for(val);
        self
    }
    #[deprecated = "Use `set_has_hidden_offscreen_nodes()` instead."]
    pub fn has_hidden_offscreen_nodes(&mut self, val: bool) -> &mut Self {
        self.set_has_hidden_offscreen_nodes(val);
        self
    }
    #[deprecated = "Use `set_scrollable()` instead."]
    pub fn scrollable(&mut self, val: bool) -> &mut Self {
        self.set_scrollable(val);
        self
    }
    #[deprecated = "Use `set_anchor_affinity()` instead."]
    pub fn anchor_affinity(&mut self, val: String) -> &mut Self {
        self.set_anchor_affinity(val);
        self
    }
    #[deprecated = "Use `set_pos_in_set()` instead."]
    pub fn pos_in_set(&mut self, val: i32) -> &mut Self {
        self.set_pos_in_set(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &Object) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_table_row_header()` instead."]
    pub fn table_row_header(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_table_row_header(val);
        self
    }
    #[deprecated = "Use `set_aria_invalid_value()` instead."]
    pub fn aria_invalid_value(&mut self, val: String) -> &mut Self {
        self.set_aria_invalid_value(val);
        self
    }
    #[deprecated = "Use `set_table_cell_aria_column_index()` instead."]
    pub fn table_cell_aria_column_index(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_aria_column_index(val);
        self
    }
    #[deprecated = "Use `set_table_cell_column_span()` instead."]
    pub fn table_cell_column_span(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_column_span(val);
        self
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: String) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_controlled_by()` instead."]
    pub fn controlled_by(&mut self, val: &Array) -> &mut Self {
        self.set_controlled_by(val);
        self
    }
    #[deprecated = "Use `set_next_sibling()` instead."]
    pub fn next_sibling(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_next_sibling(val);
        self
    }
    #[deprecated = "Use `set_next_on_line()` instead."]
    pub fn next_on_line(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_next_on_line(val);
        self
    }
    #[deprecated = "Use `set_selection_start_offset()` instead."]
    pub fn selection_start_offset(&mut self, val: i32) -> &mut Self {
        self.set_selection_start_offset(val);
        self
    }
    #[deprecated = "Use `set_set_selection()` instead."]
    pub fn set_selection(&mut self, val: &Function) -> &mut Self {
        self.set_set_selection(val);
        self
    }
    #[deprecated = "Use `set_add_event_listener()` instead."]
    pub fn add_event_listener(&mut self, val: &Function) -> &mut Self {
        self.set_add_event_listener(val);
        self
    }
    #[deprecated = "Use `set_table_cell_row_index()` instead."]
    pub fn table_cell_row_index(&mut self, val: i32) -> &mut Self {
        self.set_table_cell_row_index(val);
        self
    }
    #[deprecated = "Use `set_hit_test()` instead."]
    pub fn hit_test(&mut self, val: &Function) -> &mut Self {
        self.set_hit_test(val);
        self
    }
    #[deprecated = "Use `set_word_starts()` instead."]
    pub fn word_starts(&mut self, val: &Array) -> &mut Self {
        self.set_word_starts(val);
        self
    }
    #[deprecated = "Use `set_start_ducking_media()` instead."]
    pub fn start_ducking_media(&mut self, val: &Function) -> &mut Self {
        self.set_start_ducking_media(val);
        self
    }
    #[deprecated = "Use `set_role()` instead."]
    pub fn role(&mut self, val: RoleType) -> &mut Self {
        self.set_role(val);
        self
    }
    #[deprecated = "Use `set_stop_ducking_media()` instead."]
    pub fn stop_ducking_media(&mut self, val: &Function) -> &mut Self {
        self.set_stop_ducking_media(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Function) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_text_sel_end()` instead."]
    pub fn text_sel_end(&mut self, val: i32) -> &mut Self {
        self.set_text_sel_end(val);
        self
    }
    #[deprecated = "Use `set_remove_event_listener()` instead."]
    pub fn remove_event_listener(&mut self, val: &Function) -> &mut Self {
        self.set_remove_event_listener(val);
        self
    }
    #[deprecated = "Use `set_non_inline_text_word_ends()` instead."]
    pub fn non_inline_text_word_ends(&mut self, val: &Array) -> &mut Self {
        self.set_non_inline_text_word_ends(val);
        self
    }
    #[deprecated = "Use `set_is_image()` instead."]
    pub fn is_image(&mut self, val: bool) -> &mut Self {
        self.set_is_image(val);
        self
    }
    #[deprecated = "Use `set_value_for_range()` instead."]
    pub fn value_for_range(&mut self, val: f64) -> &mut Self {
        self.set_value_for_range(val);
        self
    }
    #[deprecated = "Use `set_aria_column_count()` instead."]
    pub fn aria_column_count(&mut self, val: i32) -> &mut Self {
        self.set_aria_column_count(val);
        self
    }
    #[deprecated = "Use `set_previous_focus()` instead."]
    pub fn previous_focus(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_previous_focus(val);
        self
    }
    #[deprecated = "Use `set_access_key()` instead."]
    pub fn access_key(&mut self, val: String) -> &mut Self {
        self.set_access_key(val);
        self
    }
    #[deprecated = "Use `set_default_action_verb()` instead."]
    pub fn default_action_verb(&mut self, val: DefaultActionVerb) -> &mut Self {
        self.set_default_action_verb(val);
        self
    }
    #[deprecated = "Use `set_find()` instead."]
    pub fn find(&mut self, val: &Function) -> &mut Self {
        self.set_find(val);
        self
    }
    #[deprecated = "Use `set_scroll_down()` instead."]
    pub fn scroll_down(&mut self, val: &Function) -> &mut Self {
        self.set_scroll_down(val);
        self
    }
    #[deprecated = "Use `set_long_click_label()` instead."]
    pub fn long_click_label(&mut self, val: String) -> &mut Self {
        self.set_long_click_label(val);
        self
    }
    #[deprecated = "Use `set_next_focus()` instead."]
    pub fn next_focus(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_next_focus(val);
        self
    }
    #[deprecated = "Use `set_class_name()` instead."]
    pub fn class_name(&mut self, val: String) -> &mut Self {
        self.set_class_name(val);
        self
    }
    #[deprecated = "Use `set_sort_direction()` instead."]
    pub fn sort_direction(&mut self, val: SortDirectionType) -> &mut Self {
        self.set_sort_direction(val);
        self
    }
    #[deprecated = "Use `set_is_selection_backward()` instead."]
    pub fn is_selection_backward(&mut self, val: bool) -> &mut Self {
        self.set_is_selection_backward(val);
        self
    }
    #[deprecated = "Use `set_standard_actions()` instead."]
    pub fn standard_actions(&mut self, val: &Array) -> &mut Self {
        self.set_standard_actions(val);
        self
    }
    #[deprecated = "Use `set_in_page_link_target()` instead."]
    pub fn in_page_link_target(&mut self, val: &AutomationNode) -> &mut Self {
        self.set_in_page_link_target(val);
        self
    }
    #[deprecated = "Use `set_active_descendant_for()` instead."]
    pub fn active_descendant_for(&mut self, val: &Array) -> &mut Self {
        self.set_active_descendant_for(val);
        self
    }
    #[deprecated = "Use `set_language()` instead."]
    pub fn language(&mut self, val: String) -> &mut Self {
        self.set_language(val);
        self
    }
    #[deprecated = "Use `set_caret_bounds()` instead."]
    pub fn caret_bounds(&mut self, val: &Rect) -> &mut Self {
        self.set_caret_bounds(val);
        self
    }
    #[deprecated = "Use `set_aria_row_count()` instead."]
    pub fn aria_row_count(&mut self, val: i32) -> &mut Self {
        self.set_aria_row_count(val);
        self
    }
}
impl Default for AutomationNode {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Get the automation tree for the whole desktop which consists of all on screen views. Note this API is currently only supported on Chrome OS.
    #[wasm_bindgen(js_namespace = ["chrome", "automation"], js_name = "getDesktop")]
    pub fn get_desktop() -> Promise;
    ///Get the automation node that currently has focus, globally. Will return null if none of the nodes in any loaded trees have focus.
    #[wasm_bindgen(js_namespace = ["chrome", "automation"], js_name = "getFocus")]
    pub fn get_focus() -> Promise;
    ///Get the automation node that currently has accessibility focus, globally. Will return null if none of the nodes in any loaded trees have accessibility focus.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "automation"],
        js_name = "getAccessibilityFocus"
    )]
    pub fn get_accessibility_focus() -> Promise;
    ///Add a tree change observer. Tree change observers are static/global, they listen to changes across all trees. Pass a filter to determine what specific tree changes to listen to, and note that listnening to all tree changes can be expensive.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "automation"],
        js_name = "addTreeChangeObserver"
    )]
    pub fn add_tree_change_observer(filter: TreeChangeObserverFilter, observer: Function);
    ///Remove a tree change observer.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "automation"],
        js_name = "removeTreeChangeObserver"
    )]
    pub fn remove_tree_change_observer(observer: Function);
    ///Sets the selection in a tree. This creates a selection in a single tree (anchorObject and focusObject must have the same root). Everything in the tree between the two node/offset pairs gets included in the selection. The anchor is where the user started the selection, while the focus is the point at which the selection gets extended e.g. when dragging with a mouse or using the keyboard. For nodes with the role staticText, the offset gives the character offset within the value where the selection starts or ends, respectively.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "automation"],
        js_name = "setDocumentSelection"
    )]
    pub fn set_document_selection(params: SetDocumentSelectionParams);
}

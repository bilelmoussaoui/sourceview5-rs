(function() {var implementors = {};
implementors["sourceview5"] = [{"text":"impl !Send for BufferBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Buffer","synthetic":true,"types":[]},{"text":"impl !Send for CompletionBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Completion","synthetic":true,"types":[]},{"text":"impl !Send for CompletionCellBuilder","synthetic":true,"types":[]},{"text":"impl !Send for CompletionCell","synthetic":true,"types":[]},{"text":"impl !Send for CompletionContextBuilder","synthetic":true,"types":[]},{"text":"impl !Send for CompletionContext","synthetic":true,"types":[]},{"text":"impl !Send for CompletionProposal","synthetic":true,"types":[]},{"text":"impl !Send for CompletionProvider","synthetic":true,"types":[]},{"text":"impl Send for CompletionSnippetsBuilder","synthetic":true,"types":[]},{"text":"impl !Send for CompletionSnippets","synthetic":true,"types":[]},{"text":"impl Send for CompletionWordsBuilder","synthetic":true,"types":[]},{"text":"impl !Send for CompletionWords","synthetic":true,"types":[]},{"text":"impl Send for FileBuilder","synthetic":true,"types":[]},{"text":"impl !Send for File","synthetic":true,"types":[]},{"text":"impl !Send for FileLoaderBuilder","synthetic":true,"types":[]},{"text":"impl !Send for FileLoader","synthetic":true,"types":[]},{"text":"impl !Send for FileSaverBuilder","synthetic":true,"types":[]},{"text":"impl !Send for FileSaver","synthetic":true,"types":[]},{"text":"impl !Send for GutterBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Gutter","synthetic":true,"types":[]},{"text":"impl !Send for GutterLines","synthetic":true,"types":[]},{"text":"impl !Send for GutterRenderer","synthetic":true,"types":[]},{"text":"impl !Send for GutterRendererPixbufBuilder","synthetic":true,"types":[]},{"text":"impl !Send for GutterRendererPixbuf","synthetic":true,"types":[]},{"text":"impl Send for GutterRendererTextBuilder","synthetic":true,"types":[]},{"text":"impl !Send for GutterRendererText","synthetic":true,"types":[]},{"text":"impl !Send for Language","synthetic":true,"types":[]},{"text":"impl Send for LanguageManagerBuilder","synthetic":true,"types":[]},{"text":"impl !Send for LanguageManager","synthetic":true,"types":[]},{"text":"impl !Send for MapBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Map","synthetic":true,"types":[]},{"text":"impl Send for MarkBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Mark","synthetic":true,"types":[]},{"text":"impl !Send for MarkAttributesBuilder","synthetic":true,"types":[]},{"text":"impl !Send for MarkAttributes","synthetic":true,"types":[]},{"text":"impl !Send for PrintCompositorBuilder","synthetic":true,"types":[]},{"text":"impl !Send for PrintCompositor","synthetic":true,"types":[]},{"text":"impl !Send for RegionBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Region","synthetic":true,"types":[]},{"text":"impl !Send for SearchContextBuilder","synthetic":true,"types":[]},{"text":"impl !Send for SearchContext","synthetic":true,"types":[]},{"text":"impl Send for SearchSettingsBuilder","synthetic":true,"types":[]},{"text":"impl !Send for SearchSettings","synthetic":true,"types":[]},{"text":"impl Send for SnippetBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Snippet","synthetic":true,"types":[]},{"text":"impl !Send for SnippetChunkBuilder","synthetic":true,"types":[]},{"text":"impl !Send for SnippetChunk","synthetic":true,"types":[]},{"text":"impl !Send for SnippetContext","synthetic":true,"types":[]},{"text":"impl Send for SnippetManagerBuilder","synthetic":true,"types":[]},{"text":"impl !Send for SnippetManager","synthetic":true,"types":[]},{"text":"impl Send for SpaceDrawerBuilder","synthetic":true,"types":[]},{"text":"impl !Send for SpaceDrawer","synthetic":true,"types":[]},{"text":"impl Send for StyleBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Style","synthetic":true,"types":[]},{"text":"impl Send for StyleSchemeBuilder","synthetic":true,"types":[]},{"text":"impl !Send for StyleScheme","synthetic":true,"types":[]},{"text":"impl !Send for StyleSchemeChooser","synthetic":true,"types":[]},{"text":"impl !Send for StyleSchemeChooserButton","synthetic":true,"types":[]},{"text":"impl !Send for StyleSchemeChooserWidget","synthetic":true,"types":[]},{"text":"impl Send for StyleSchemeManagerBuilder","synthetic":true,"types":[]},{"text":"impl !Send for StyleSchemeManager","synthetic":true,"types":[]},{"text":"impl !Send for TagBuilder","synthetic":true,"types":[]},{"text":"impl !Send for Tag","synthetic":true,"types":[]},{"text":"impl !Send for ViewBuilder","synthetic":true,"types":[]},{"text":"impl !Send for View","synthetic":true,"types":[]},{"text":"impl !Send for Encoding","synthetic":true,"types":[]},{"text":"impl Send for FileSaverFlags","synthetic":true,"types":[]},{"text":"impl Send for SortFlags","synthetic":true,"types":[]},{"text":"impl Send for SpaceLocationFlags","synthetic":true,"types":[]},{"text":"impl Send for SpaceTypeFlags","synthetic":true,"types":[]},{"text":"impl Send for BackgroundPatternType","synthetic":true,"types":[]},{"text":"impl Send for BracketMatchType","synthetic":true,"types":[]},{"text":"impl Send for ChangeCaseType","synthetic":true,"types":[]},{"text":"impl Send for CompletionActivation","synthetic":true,"types":[]},{"text":"impl Send for CompletionColumn","synthetic":true,"types":[]},{"text":"impl Send for CompressionType","synthetic":true,"types":[]},{"text":"impl Send for FileLoaderError","synthetic":true,"types":[]},{"text":"impl Send for FileSaverError","synthetic":true,"types":[]},{"text":"impl Send for GutterRendererAlignmentMode","synthetic":true,"types":[]},{"text":"impl Send for NewlineType","synthetic":true,"types":[]},{"text":"impl Send for SmartHomeEndType","synthetic":true,"types":[]},{"text":"impl Send for ViewGutterPosition","synthetic":true,"types":[]},{"text":"impl Send for BufferClass","synthetic":false,"types":[]},{"text":"impl Send for CompletionClass","synthetic":false,"types":[]},{"text":"impl Send for CompletionCellClass","synthetic":false,"types":[]},{"text":"impl Send for CompletionContextClass","synthetic":false,"types":[]},{"text":"impl Send for CompletionSnippetsClass","synthetic":false,"types":[]},{"text":"impl Send for CompletionWordsClass","synthetic":false,"types":[]},{"text":"impl Send for FileClass","synthetic":false,"types":[]},{"text":"impl Send for FileLoaderClass","synthetic":false,"types":[]},{"text":"impl Send for FileSaverClass","synthetic":false,"types":[]},{"text":"impl Send for GutterClass","synthetic":false,"types":[]},{"text":"impl Send for GutterLinesClass","synthetic":false,"types":[]},{"text":"impl Send for GutterRendererClass","synthetic":false,"types":[]},{"text":"impl Send for GutterRendererPixbufClass","synthetic":false,"types":[]},{"text":"impl Send for GutterRendererTextClass","synthetic":false,"types":[]},{"text":"impl Send for LanguageClass","synthetic":false,"types":[]},{"text":"impl Send for LanguageManagerClass","synthetic":false,"types":[]},{"text":"impl Send for MapClass","synthetic":false,"types":[]},{"text":"impl Send for MarkClass","synthetic":false,"types":[]},{"text":"impl Send for MarkAttributesClass","synthetic":false,"types":[]},{"text":"impl Send for PrintCompositorClass","synthetic":false,"types":[]},{"text":"impl Send for RegionClass","synthetic":false,"types":[]},{"text":"impl Send for SearchContextClass","synthetic":false,"types":[]},{"text":"impl Send for SearchSettingsClass","synthetic":false,"types":[]},{"text":"impl Send for SnippetClass","synthetic":false,"types":[]},{"text":"impl Send for SnippetChunkClass","synthetic":false,"types":[]},{"text":"impl Send for SnippetContextClass","synthetic":false,"types":[]},{"text":"impl Send for SnippetManagerClass","synthetic":false,"types":[]},{"text":"impl Send for SpaceDrawerClass","synthetic":false,"types":[]},{"text":"impl Send for StyleClass","synthetic":false,"types":[]},{"text":"impl Send for StyleSchemeClass","synthetic":false,"types":[]},{"text":"impl Send for StyleSchemeChooserButtonClass","synthetic":false,"types":[]},{"text":"impl Send for StyleSchemeChooserWidgetClass","synthetic":false,"types":[]},{"text":"impl Send for StyleSchemeManagerClass","synthetic":false,"types":[]},{"text":"impl Send for TagClass","synthetic":false,"types":[]},{"text":"impl Send for ViewClass","synthetic":false,"types":[]}];
implementors["sourceview5_sys"] = [{"text":"impl !Send for GtkSourceBufferClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionCellClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionContextClass","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionProposalInterface","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionProviderInterface","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionSnippetsClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionWordsClass","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceEncoding","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceFileClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceFileLoaderClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceFileSaverClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterLinesClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRendererClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRendererPixbufClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRendererTextClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceLanguageClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceLanguageManagerClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceMapClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceMarkAttributesClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceMarkClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourcePrintCompositorClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceRegionClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceRegionIter","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSearchContextClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSearchSettingsClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSnippetChunkClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSnippetClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSnippetContextClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSnippetManagerClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSpaceDrawerClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeChooserButtonClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeChooserInterface","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeChooserWidgetClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeManagerClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceTagClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceViewClass","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceBuffer","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletion","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionCell","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionContext","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionSnippets","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceCompletionWords","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceFile","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceFileLoader","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceFileSaver","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceGutter","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceGutterLines","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRenderer","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRendererPixbuf","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceGutterRendererText","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceLanguage","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceLanguageManager","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceMap","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceMark","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceMarkAttributes","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourcePrintCompositor","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceRegion","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSearchContext","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceSearchSettings","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSnippet","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSnippetChunk","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSnippetContext","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSnippetManager","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceSpaceDrawer","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceStyle","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceStyleScheme","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeChooserButton","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceStyleSchemeChooserWidget","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceStyleSchemeManager","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceTag","synthetic":true,"types":[]},{"text":"impl !Send for GtkSourceView","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionProposal","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceCompletionProvider","synthetic":true,"types":[]},{"text":"impl Send for GtkSourceStyleSchemeChooser","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
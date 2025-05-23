# Changelog

## [0.2.1] - 2025-05-18

### Changed
- Updated file extension for VQL storage files from .vql to .vql.ref to better reflect their purpose as reference files
- Standardized file handling throughout the codebase

## [0.2.0] - 2025-05-18

### Added
- Dynamic asset type support - now allows custom asset types beyond the default three (models, controllers, UI)
- Automatic VQL reference file creation for custom asset types
- Helper function to map short type names to full type names
- Expanded reference documentation with asset type information
- Test suite for asset type functionality

### Changed
- Modified `add_asset_reference` to support any alphabetic type character
- Updated `get_asset_file_path` to dynamically determine file paths based on type
- Simplified `get_asset_type_from_ref` to accept any alphabetic character
- Improved documentation throughout the codebase
- Enhanced README with information about dynamic asset types

## [0.1.0] - 2025-05-01

### Added
- Initial version of VQL CLI
- Support for default asset types (models, controllers, UI)
- Basic VQL file manipulation
- Asset reference and entity management
- Review storage functionality
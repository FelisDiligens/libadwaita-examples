sources = [
  'main.vala',
  'window.vala',
]

deps = [
  dependency ('gtk4'),
  dependency ('libadwaita-1'),
]

sources += gnome.compile_resources (
  'gresources',
  'gresources.xml',
  source_dir: ['src/'],
)

executable ('load-template-vala', sources,
  dependencies: deps,
     vala_args: [ '--gresourcesdir=src/' ],
       install: true,
)
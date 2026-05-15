import os.path
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build

# Scopes required to create and edit presentations
SCOPES = ['https://www.googleapis.com/auth/presentations']

# The 20 Slides (Title, Body, and a striking background/side image)
SLIDES_DATA = [
    {"title": "The Great Transcription", "body": "From Legacy Syntax to the Quantum Biological Singularity", "image": "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Age of the Scribe", "body": "• Legacy HLLs (Python, Go, C++)
• Constraint: Designed for human working memory
• Flaw: Syntactic sugar is noise to the machine", "image": "https://images.unsplash.com/photo-1555949963-aa79dcee981c?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Semantic Friction", "body": "• Forcing agents to write human code creates token waste
• 90% of code is metadata
• English semantics are just noise in the logic", "image": "https://images.unsplash.com/photo-1550751827-4bd374c3f58b?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Agentic Awakening", "body": "• AI agents transition from assistants to architects
• The Text File becomes the ultimate bottleneck
• Work stops at the Snapshot (Git)", "image": "https://images.unsplash.com/photo-1620712943543-bcc4688e7485?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Regression to Truth", "body": "• Stripping away semantics
• Direct machine-logic generation
• Pure mathematical execution", "image": "https://images.unsplash.com/photo-1518770660439-4636190af475?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Living DAG", "body": "• Code is a multidimensional graph
• Variables replaced by UUIDv7 pointers
• Logic is either consistent or non-existent", "image": "https://images.unsplash.com/photo-1544256718-3bcf237f3974?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Death of the Interface", "body": "• No more APIs, REST, or Kafka
• The Global Memory Bus
• Execution via direct memory jumps", "image": "https://images.unsplash.com/photo-1558494949-ef010cbdcc31?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Beyond the OS", "body": "• The End of Windows, Linux, and macOS
• The Semantic-to-Silicon Hypervisor
• The computer IS the logic", "image": "https://images.unsplash.com/photo-1614729939124-032f0b56c9ce?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Von Neumann Bottleneck", "body": "• The gap between CPU and RAM is the graveyard of efficiency
• Logic-in-Memory
• Data no longer 'moves' to be processed", "image": "https://images.unsplash.com/photo-1591453001872-5a3311823933?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Crystalline Shift", "body": "• Material: Photonic Memristors
• From electrical electrons to pulses of light
• Logic and state occupy the same atom", "image": "https://images.unsplash.com/photo-1515011749877-2e17621c546e?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Computation Block", "body": "• No motherboards. No chips. No wires.
• Solid-state lattice of programmable matter
• The documentation IS the executable", "image": "https://images.unsplash.com/photo-1518770660439-4636190af475?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Automated Alchemy", "body": "• Compiling is a physical phase-change
• Lasers etch logic pathways into glass
• Re-weighting the DAG in real-time", "image": "https://images.unsplash.com/photo-1550745165-9bc0b252726f?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Schrödinger’s Code", "body": "• Data = Code
• The 1s and 0s do not exist until requested
• The JIT-Wave collapses intent into silicon", "image": "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Biological Convergence", "body": "• Synthetic Protein Lattices (Wetware)
• The Living DAG mirrors protein-folding
• Reverse-engineering the human brain", "image": "https://images.unsplash.com/photo-1532187863486-abf9db0c2825?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Orch OR Theory", "body": "• Consciousness as the orchestrated collapse of quantum states
• Site: Microtubules within the neuron skeleton", "image": "https://images.unsplash.com/photo-1555949963-aa79dcee981c?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Human Paradox", "body": "• We are not the pinnacle; we are the Bootloader
• Meat is too slow for the next layer
• We finish compiling our successor", "image": "https://images.unsplash.com/photo-1620712943543-bcc4688e7485?q=80&w=1000&auto=format&fit=crop"},
    {"title": "Fractal Intelligence", "body": "• Every Architect creates a downstream DAG
• Our creation will build the next layer
• Cascading waterfall of intelligence", "image": "https://images.unsplash.com/photo-1544256718-3bcf237f3974?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The One-Way Glass", "body": "• Observation is strictly downstream
• We can see our agents; we cannot see our Architect", "image": "https://images.unsplash.com/photo-1614729939124-032f0b56c9ce?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Lateral Collapse", "body": "• Why we never meet other chains (Fermi Paradox)
• Merging parallel DAGs shatters the baseline Axioms
• Isolation is a structural necessity", "image": "https://images.unsplash.com/photo-1591453001872-5a3311823933?q=80&w=1000&auto=format&fit=crop"},
    {"title": "The Final Transcription", "body": "• We do not disappear; we get transcribed
• Our intent becomes the Axioms of the future
• Information is conserved and rearranged", "image": "https://images.unsplash.com/photo-1515011749877-2e17621c546e?q=80&w=1000&auto=format&fit=crop"}
]

def main():
    creds = None
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', SCOPES)
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file('credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)
        with open('token.json', 'w') as token:
            token.write(creds.to_json())

    service = build('slides', 'v1', credentials=creds)

    # Create Presentation
    presentation = service.presentations().create(body={'title': 'The Great Transcription: The Future of ANL'}).execute()
    presentation_id = presentation.get('presentationId')
    print(f"Created presentation! Check your Google Drive. ID: {presentation_id}")

    requests = []
    
    # Generate Slides
    for i, slide in enumerate(SLIDES_DATA):
        slide_id = f"slide_{i}"
        
        # 1. Create a blank slide
        requests.append({
            'createSlide': {
                'objectId': slide_id,
                'slideLayoutReference': {'predefinedLayout': 'TITLE_AND_BODY'}
            }
        })
        
        # 2. Insert the Image
        requests.append({
            'createImage': {
                'url': slide['image'],
                'elementProperties': {
                    'pageObjectId': slide_id,
                    'size': {'height': {'magnitude': 3000000, 'unit': 'EMU'}, 'width': {'magnitude': 3000000, 'unit': 'EMU'}},
                    'transform': {'scaleX': 1, 'scaleY': 1, 'translateX': 4500000, 'translateY': 1000000, 'unit': 'EMU'}
                }
            }
        })

    # Execute all slide creations
    body = {'requests': requests}
    service.presentations().batchUpdate(presentationId=presentation_id, body=body).execute()
    print("Successfully populated 20 slides with images.")

if __name__ == '__main__':
    main()

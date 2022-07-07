use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub enum MovieInstruction {
  AddMovieReview {
    title: String,
    rating: u8,
    description: String
  },
  UpdateMovieReview {
    title: String,
    rating: u8,
    description: String
  },
  AddComment {
    review: Pubkey,
    comment: String
  }
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
  title: String,
  rating: u8,
  description: String
}

#[derive(BorshDeserialize)]
struct CommentPayload {
    review: Pubkey,
    comment: String
}


impl MovieInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description }
            },
                
            1 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::UpdateMovieReview {
                    title: payload.title,
                    rating: payload.rating,
                    description: payload.description 
                }
            },
            2 => {
                let payload = CommentPayload::try_from_slice(rest).unwrap();
                Self::AddComment {
                    review: payload.review,
                    comment: payload.comment
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}
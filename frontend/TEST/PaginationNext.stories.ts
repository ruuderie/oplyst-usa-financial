import type { Meta, StoryObj } from '@storybook/vue3';

import PaginationNext from '../components/ui/pagination/PaginationNext.vue';

const meta = {
  title: 'PaginationNext',
  component: PaginationNext,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PaginationNext>;

export default meta;
type Story = StoryObj<typeof PaginationNext>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
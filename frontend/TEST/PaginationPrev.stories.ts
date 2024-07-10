import type { Meta, StoryObj } from '@storybook/vue3';

import PaginationPrev from '../components/ui/pagination/PaginationPrev.vue';

const meta = {
  title: 'PaginationPrev',
  component: PaginationPrev,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PaginationPrev>;

export default meta;
type Story = StoryObj<typeof PaginationPrev>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
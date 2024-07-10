import type { Meta, StoryObj } from '@storybook/vue3';

import PopoverContent from '../components/ui/popover/PopoverContent.vue';

const meta = {
  title: 'PopoverContent',
  component: PopoverContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PopoverContent>;

export default meta;
type Story = StoryObj<typeof PopoverContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
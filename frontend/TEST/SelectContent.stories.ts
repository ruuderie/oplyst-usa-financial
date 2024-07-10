import type { Meta, StoryObj } from '@storybook/vue3';

import SelectContent from '../components/ui/select/SelectContent.vue';

const meta = {
  title: 'SelectContent',
  component: SelectContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectContent>;

export default meta;
type Story = StoryObj<typeof SelectContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};